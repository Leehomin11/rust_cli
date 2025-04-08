use clap::Parser;
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = 25)]
    focus: u64,

    #[arg(short, long, default_value_t = 5)]
    breaktime: u64,

    #[arg(short, long, default_value_t = 1)]
    rounds: u64,
}

fn main() {
    let args = Args::parse();

    for round in 1..=args.rounds {
        println!("✔️ 라운드 {}/{}", round, args.rounds);
        
        println!("🔥 집중 시작: {}분", args.focus);
        run_timer(args.focus * 60, "⏰ 집중 중...");

        if round != args.rounds {
            println!("💤 쉬는 시간: {}분", args.breaktime);
            run_timer(args.breaktime * 60, "😴 쉬는 중...");
        }

        println!("✅ 라운드 {}/{} 완료!\n", round, args.rounds);
    }

    println!("🎉 모든 라운드를 완료했습니다! 수고하셨어요 👏");
}

fn run_timer(seconds: u64, label: &str) {
    for remaining in (0..seconds).rev() {
        print!("\r{} 남은 시간: {:02}:{:02}", label, remaining / 60, remaining % 60);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_secs(1));
    }

    println!("\r🔔 시간 종료!");
}
