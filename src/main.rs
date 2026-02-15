// 標準ライブラリのインポート。Writeはファイル書き込みを可能とするtrait
use std::io::{self, Write};
// Commandはプロセスを操るための構造体。
// Stdioはプロセスの入出力を制御するための構造体。出力をRustに渡したり、コンソールに表示したり、そもそも表示させなかったり、、、などの制御ができる。
use std::process::{Command, Stdio};

// anyhowはエラー処理の便利クレーと。Contextはエラー発生時に説明文を表示させるためのtraitで、Resultは型エイリアス、bailはその場でエラーを発生させて関数から脱出するためのマクロ。
use anyhow::{Context, Result, bail};

// コマンドの引数をArgsの構造体にパースするためのクレート。Parserはコマンドライン引数を構造体に変換するためのtrait。
use clap::Parser;
// 一行での編集を実現するためのエディタライブラリ。DefaultEditorはデフォルトのエディタを提供する構造体で、ReadlineErrorはユーザー入力のエラーを表す列挙型。
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

// ここでParserを渡すことで、Args構造体がコマンドライン引数パーサを"自動的に"(←ここ重要)作成するようになる。
#[derive(Parser, Debug)]

// 見ての通り
#[command(
    name = "gtc",
    version,
    about = "コミットメッセージを英語に翻訳するためのコマンドです。",
)]

// 入力された文字列の引数構造体
struct Args {
    message: String,
}

fn main() {
    // 見ての通り
    if let Err(err) = run() {
        eprintln!("Error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // gitリポジトリかどうかを判定。いつもの?があるので、エラー時は即return。
    ensure_git_repo()?;

    // 英語に翻訳を行う。
    let translated = translate_to_english(&args.message)?;
    // エディターを起動。翻訳されたメッセージを初期値として渡しつつ、編集することを可能に。
    let edited = edit_message(&translated)?;
    // 見ての通り
    if edited.trim().is_empty() {
        bail!("編集後のコミットメッセージが空です。中止します。");
    }

    // 翻訳した内容をもとに、ユーザーが編集した内容でコミットを実行。
    commit_with_message(&edited)
}

// gitリポジトリかどうかを判定するための関数。rev-parseを呼び出して、成功すればgitリポジトリ、失敗すればそうでないと判断する。
fn ensure_git_repo() -> Result<()> {
    let status = Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null()) // コマンドの標準出力を捨てる。
        // コマンドの標準エラー出力も捨てる。これで、コマンドが成功しても失敗しても、ユーザーには何も表示されない。
        // じゃあ、どうやって成功失敗を判断しているかというと、exit codeで判断してます。この下のif文で確認済み。
        .stderr(Stdio::null())
        .status()
        .context("gitコマンドの実行に失敗しました。gitがインストールされているか確認してください。")?;

    // 
    if !status.success() {
        bail!("gitリポジトリではありません。");
    }
    Ok(())
}

// 翻訳処理を行うためのtransコマンドを呼び出している。
fn translate_to_english(message_ja: &str) -> Result<String> {
    // 見ての通り
    let output = Command::new("trans")
        .args(["-b", ":en", message_ja])
        .output()
        .context("翻訳に失敗しました。transコマンドがインストールされているか確認してください。")?;

    // 失敗時。
    if !output.status.success() {
        // 失敗時にfrom_utf8_lossyを使用する理由はエラー内容をとにかく表示するため。翻訳に失敗した理由がUTF-8エラーだった場合でも、ユーザーに何が起こったのかを伝えるために、エラー出力を文字列として表示させる。
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("翻訳に失敗しました。: {}", stderr.trim());
    }

    // 成功時。
    let translated = String::from_utf8(output.stdout).context("翻訳結果が有効なUTF-8ではありません。")?;
    let translated = translated.trim().to_string();
    if translated.is_empty() {
        bail!("翻訳結果が空です。");
    }

    Ok(translated)
}

fn edit_message(initial: &str) -> Result<String> {
    let mut editor = DefaultEditor::new().context("ターミナルの初期化に失敗しました。")?;
    
    // これ以降の文字色を黄色に変更。入力行がみやすいように。
    print!("\x1b[33m");

    // 黄色に変更した内容を即時反映させるためにflush。
    io::stdout().flush().context("出力に失敗しました。")?;

    // エディタを起動して、ユーザーに編集させる。初期値として翻訳されたメッセージを渡す。
    let edited = match editor.readline_with_initial("", (initial, "")) {
        Ok(line) => line,
        Err(ReadlineError::Interrupted | ReadlineError::Eof) => bail!("処理を中止しました。"),
        Err(err) => return Err(err).context("エディタの起動に失敗しました。"),
    };
    print!("\x1b[0m");
    io::stdout().flush().context("出力に失敗しました。")?;

    Ok(edited.trim().to_string())
}

// 見ての通り。
fn commit_with_message(message_en: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["commit", "-m", message_en])
        .status()
        .context("git commitコマンドの実行に失敗しました。")?;

    if !status.success() {
        bail!("git commitに失敗しました。");
    }
    Ok(())
}
