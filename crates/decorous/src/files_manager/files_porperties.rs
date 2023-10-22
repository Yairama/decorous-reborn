use std::path::Path;


pub trait FileProperties{

    fn path(&self) -> String;

    fn name_with_extension(&self) -> Option<String> {
        let ruta = self.path().clone().to_string();
        let path = Path::new(ruta.as_str());
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name1| name1.to_string())
    }

    fn name(&self) -> Option<String> {
        let ruta = self.path().clone().to_string();
        let path = Path::new(ruta.as_str());
        path.file_stem()
            .and_then(|stem| stem.to_str())
            .map(|name| name.to_string())
    }

}
