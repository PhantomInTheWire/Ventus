use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
#[allow(dead_code)]
pub enum ResultCode {
    RestartMarkerReply = 110,
    ServiceReadyInXXXXMinutes = 120,
    FileStatusOk = 125,
    Ok = 200,
    CommandNotImplementedSuperfluousAtThisSite = 202,
    SystemStatus = 211,
    DirectoryStatus = 212,
    FileStatus = 213,
    HelpMessage = 214,
    SystemType = 215,
    ServiceReadyForNewUser = 220,
    ServiceClosingControlConnection = 221,
    DataConnectionOpen = 225,
    ClosingDataConnection = 226,
    EnteringPassiveMode = 227,
    UserLoggedIn = 230,
    RequestedFileActionOkay = 250,
    PATHNAMECreated = 257,
    NeedAccountForLogin = 331,
    UserAccountForLogin = 332,
    RequestFurtherInformation = 350,
    ServiceNotAvailable = 421,
    CantOpenDataConnection = 425,
    ConnectionClosed = 426,
    LocalErrorInProcessing = 451,
    InsufficientStorageSpace = 452,
    UnknownCommand = 500,
    InvalidParameterOrArgument = 501,
    BadSequenceOfCommands = 503,
    CommandNotImplemented = 502,
    CommandNotImplementedForThatParameter = 504,
    NotLoggedIn = 530,
    NeedAccountForStoringFiles = 532,
    PageTypeUnknown = 551,
    FileNameNotAllowed = 553,
    OpeningDataConnection = 150,
    FileActionNotTaken = 450,
    FileUnavailable = 550,
}

#[derive(Clone, Debug)]
pub enum Command {
    Auth,
    Syst,
    User(String),
    Pwd,
    Type,
    List(Option<PathBuf>),
    Pasv,
    Cwd(PathBuf),
    Cdup,
    Mkdir(PathBuf),
    Rmd(PathBuf),
    Stor(PathBuf),
    Retr(PathBuf),
    Unknown(String),
}

impl AsRef<str> for Command {
    fn as_ref(&self) -> &str {
        match *self {
            Command::Auth => "AUTH",
            Command::Syst => "SYST",
            Command::User(_) => "USER",
            Command::Pwd => "PWD",
            Command::Type => "TYPE",
            Command::List(_) => "LIST",
            Command::Pasv => "PASV",
            Command::Cwd(_) => "CWD",
            Command::Cdup => "CDUP",
            Command::Mkdir(_) => "MKD",
            Command::Rmd(_) => "RMD",
            Command::Unknown(_) => "UNKN",
            Command::Stor(_) => "STOR",
            Command::Retr(_) => "RETR",
        }
    }
}

impl Command {
    pub fn new(input: Vec<u8>) -> std::io::Result<Self> {
        let mut iter = input.split(|&byte| byte == b' ');
        let command = iter.next().expect("command in input").to_vec();
        let command_lowercase: Vec<u8> = command.to_ascii_lowercase();
        let data = iter.next();

        let command = match command_lowercase.as_slice() {
            b"auth" => Command::Auth,
            b"syst" => Command::Syst,
            b"user" => Command::User(data.map(|bytes| String::from_utf8_lossy(bytes).to_string()).unwrap_or_default()),
            b"pwd" => Command::Pwd,
            b"type" => Command::Type,
            b"list" => Command::List(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string()))),
            b"pasv" => Command::Pasv,
            b"cwd" => Command::Cwd(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"cdup" => Command::Cdup,
            b"mkd" => Command::Mkdir(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"rmd" => Command::Rmd(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"stor" => Command::Stor(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            b"retr" => Command::Retr(data.map(|bytes| PathBuf::from(String::from_utf8_lossy(bytes).to_string())).unwrap_or_default()),
            _ => Command::Unknown(String::from_utf8_lossy(&command).to_string()),
        };

        Ok(command)
    }
}
