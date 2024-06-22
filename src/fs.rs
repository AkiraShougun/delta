
pub fn create_file(content:&String,filename:&String){
    let path = format!("./{0}",filename); 
    _ = std::fs::write(path, content);
}