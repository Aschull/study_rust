pub mod rs {
    #[get("/ola")]
    pub fn ola() -> &'static str {
        "Hello, world!"
    }
    
    // #[get("/hello/<name>")]
    // pub fn hello(name: String) -> String {
    //     format!("Hello, {}!", name)
    // }

}
