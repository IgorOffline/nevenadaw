fn main() {
    let raw_prefix = r"
       IDENTIFICATION DIVISION.
       PROGRAM-ID. dontmindmeimjustcobol.
       DATA DIVISION.
       WORKING-STORAGE SECTION.";
    let _raw_variables = r"
       01 WS1-FIRST PIC 9999999999 VALUE 200.
       01 WS2-SECOND PIC 9999999999 VALUE 300.
       01 WS3-SUM PIC 9999999999 VALUE 0.";
    let raw_suffix = r"
       PROCEDURE DIVISION.
           DISPLAY WS1-FIRST.
           DISPLAY WS2-SECOND.
           ADD WS1-FIRST TO WS3-SUM.
           ADD WS2-SECOND TO WS3-SUM.
           DISPLAY WS3-SUM.
           STOP RUN.";
    println!("--- --- ---");
    println!("{}", raw_prefix);
    let limit = 1000;
    for i in 1..=limit {
        let i_val = i * 100;
        print!(
            "       01 WS{}-DONT-MIND-ME-IM-JUST-COBOL PIC 9999999999 VALUE {}.",
            i, i_val
        );
        if i < limit {
            println!();
        }
    }
    println!("{}", raw_suffix);
    println!("--- --- ---");
}
