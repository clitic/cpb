use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path;

use clap::Parser;
use glob::glob;
use kdam::tqdm;
use walkdir::WalkDir;

#[cfg(feature = "gui")]
use cpb::gui_dialog;

/// Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY with a progress bar.
#[derive(Parser, Debug)]
#[clap(version, author = "clitic <clitic21@gmail.com>", about)]
struct Args {
    /// List of sources to copy.
    #[clap(required = true)]
    source: Vec<String>,

    /// Destination path or directory.
    #[clap(required = true)]
    dest: String,

    /// Copy chunk size.
    #[clap(short, long, default_value_t = 8192)]
    chunk_size: u64,

    /// Disable progress bar.
    #[clap(short, long, takes_value = false)]
    no_progress: bool,

    /// Show a gui dialog instead of terminal output.
    #[cfg(feature = "gui")]
    #[clap(short, long)]
    gui: bool,
}

// #[derive(Debug)]
struct CopyCore {
    args: Args,
    pb: kdam::Bar,
    total_size: u64,

    #[cfg(feature = "gui")]
    pub pb_gui: Option<gui_dialog::DialogUI>,
}

impl CopyCore {
    fn new(args: Args) -> Self {
        let mut pb = tqdm!(
            desc = "scanning files".to_string(),
            unit_scale = true,
            unit_divisor = 1024,
            unit = "B".to_string()
        );

        if args.no_progress {
            pb.disable = true;
        }

        #[cfg(feature = "gui")]
        {
            let mut pb_gui = None;

            if args.gui {
                pb.disable = true;
                pb_gui = Some(gui_dialog::build_ui());
            }

            CopyCore {
                args: args,
                pb: pb,
                total_size: 0,
                pb_gui: pb_gui,
            }
        }

        #[cfg(not(feature = "gui"))]
        CopyCore {
            args: args,
            pb: pb,
            total_size: 0,
        }
    }

    #[cfg(feature = "gui")]
    fn update_gui(&mut self, file_path: &str) {
        if self.pb_gui.is_some() {
            let elapsed_time_now = self.pb.internal.timer.elapsed().as_secs_f64();
            let mininterval_constraint =
                self.pb.mininterval <= (elapsed_time_now - self.pb.internal.elapsed_time);

            if mininterval_constraint || self.pb.n >= self.pb.total {
                let n = (self.pb.n as f64 / self.pb.total as f64) * 100.0;

                let mut pb_gui = self.pb_gui.as_ref().unwrap().clone();
                pb_gui.set_value(n as u32);

                self.pb.internal.elapsed_time = self.pb.internal.timer.elapsed().as_secs_f64();
                self.pb.internal.its_per = self.pb.n as f64 / self.pb.internal.elapsed_time;
                let mut remaning_time = 0.0;

                if self.pb.n >= self.pb.total {
                    pb_gui.set_value(101);
                } else {
                    remaning_time = (self.pb.total - self.pb.n) as f64 / self.pb.internal.its_per;
                }

                let text = format!(
                    "Copying: {}\nProgress: {:.2}%\nData Transferred: {}B/{}B\nElapsed Time: {}\nRemaining Time: {}\nSpeed: {}B/s",
                    path::Path::new(file_path).file_name().unwrap().to_str().unwrap(),
                    n,
                    kdam::format::format_sizeof(self.pb.n, self.pb.unit_divisor),
                    kdam::format::format_sizeof(self.pb.total, self.pb.unit_divisor),
                    kdam::format::format_interval(self.pb.internal.elapsed_time as u64),
                    kdam::format::format_interval(remaning_time as u64),
                    kdam::format::format_sizeof(self.pb.internal.its_per as u64, self.pb.unit_divisor)
                );

                pb_gui.set_text(&mut text.split("\n"));
            }
        }
    }

    fn run(&mut self) {
        if !self.args.no_progress {
            #[cfg(feature = "gui")]
            if self.args.gui {
                self.pb_gui.as_ref().unwrap().clone().set_text_label1(
                    format!("Scanning files for {}", self.args.source.join(", ")).as_str(),
                );
            }

            self.pb.refresh();

            for source in self.args.source.clone() {
                let source_one = source.as_str();
                let source_path = path::Path::new(source_one);

                if source_path.is_file() {
                    self.total_size += fs::metadata(source_one).unwrap().len();
                    self.pb.update(1);
                } else if source_path.is_dir() {
                    self.total_dir(source_one);
                } else {
                    self.total_glob(source_one);
                }
            }

            self.pb.refresh();
            println!();
            self.pb.desc = "".to_string();
            self.pb.total = self.total_size;
            self.pb.refresh();
        }

        for source in self.args.source.clone() {
            let source_one = source.as_str();
            let source_path = path::Path::new(source_one);

            if source_path.is_file() {
                let dest = self.args.dest.clone();
                self.copy_in_chunks(source_one, dest.as_str());
            } else if source_path.is_dir() {
                self.copy_dir(source_one, true);
            } else {
                self.copy_glob(source_one);
            }
        }

        println!();
    }

    fn total_dir(&mut self, source: &str) {
        for entry in WalkDir::new(source) {
            let entry_file = entry.as_ref().unwrap().path();

            if entry_file.is_file() {
                self.total_size += entry_file.metadata().unwrap().len();
                self.pb.update(1);
            }
        }
    }

    fn total_glob(&mut self, source: &str) {
        for entry in glob(source).expect("Failed to read glob pattern") {
            if let Ok(entry) = entry {
                if entry.is_file() {
                    self.total_size += entry.metadata().unwrap().len();
                    self.pb.update(1);
                } else if entry.is_dir() {
                    self.total_dir(entry.to_str().unwrap());
                }
            }
        }
    }

    fn copy_in_chunks(&mut self, source: &str, dest: &str) {
        // println!("{}\t{}", source, dest);
        let dest_file = path::Path::new(dest);

        if dest_file.is_dir() {
            panic!("USE A FILE PATH IN DEST");
        }

        fs::create_dir_all(dest_file.parent().unwrap()).unwrap();

        let src_file = fs::File::open(source).unwrap();
        let mut dst_file = fs::File::create(dest).unwrap();

        let mut reader = BufReader::new(src_file);

        loop {
            let mut chunk = vec![];
            reader
                .by_ref()
                .take(self.args.chunk_size)
                .read_to_end(&mut chunk)
                .unwrap();
            let chunk_len = chunk.len();
            dst_file.write(&chunk).unwrap();
            dst_file.flush().unwrap();

            self.pb.update(chunk_len as u64);

            #[cfg(feature = "gui")]
            self.update_gui(source);

            if chunk_len == 0 {
                self.pb.refresh();
                break;
            }
        }
    }

    fn copy_dir(&mut self, source: &str, inplace: bool) {
        for entry in WalkDir::new(source) {
            let entry_path = entry.as_ref().unwrap().path();
            let entry_path_str = entry_path.to_str().unwrap();

            let join_path;

            if entry_path_str.starts_with(".") {
                join_path = cpb::path::join(self.args.dest.clone(), entry_path_str.to_string());
            } else {
                if inplace {
                    join_path = cpb::path::join(
                        self.args.dest.clone(),
                        entry_path_str.replacen(source, "", 1),
                    );
                } else {
                    join_path = cpb::path::join(
                        self.args.dest.clone(),
                        entry_path_str.replacen(
                            cpb::path::glob_dir(self.args.source[0].clone()).as_str(),
                            ".",
                            1,
                        ),
                    );
                }
            }

            if entry_path.is_file() {
                self.copy_in_chunks(entry_path_str, join_path.as_str());
            } else if entry_path.is_dir() {
                fs::create_dir_all(join_path).unwrap();
            }
        }
    }

    fn copy_glob(&mut self, source: &str) {
        for entry in glob(source).expect("Failed to read glob pattern") {
            if let Ok(entry) = entry {
                if entry.is_file() {
                    let entry_path = entry.to_str().unwrap();

                    if entry_path.starts_with(".") {
                        self.copy_in_chunks(
                            entry_path,
                            cpb::path::join(self.args.dest.clone(), entry_path.to_string())
                                .as_str(),
                        );
                    } else {
                        self.copy_in_chunks(
                            entry_path,
                            cpb::path::join(
                                self.args.dest.clone(),
                                entry_path.replacen(
                                    cpb::path::glob_dir(self.args.source[0].clone()).as_str(),
                                    ".",
                                    1,
                                ),
                            )
                            .as_str(),
                        );
                    }
                } else if entry.is_dir() {
                    self.copy_dir(entry.to_str().unwrap(), false);
                }
            }
        }
    }
}

fn main() {
    #[cfg(not(feature = "gui"))]
    CopyCore::new(Args::parse()).run();

    #[cfg(feature = "gui")]
    {
        let mut core = CopyCore::new(Args::parse());

        if core.args.gui {
            let ui = core.pb_gui.as_ref().unwrap().ui.clone();

            std::thread::spawn(move || {
                core.run();
                std::process::exit(0);
            });

            ui.main();
        } else {
            core.run();
        }
    }
}
