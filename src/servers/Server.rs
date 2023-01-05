pub struct Server{
    port: u16,
    status: ServerStatus,
    name: String,
    category: u16,
    container_name: String
}

impl Server{
    pub fn new(port: u16, name: String, category: u16, container_name: String) -> Server{
        Server{
            port,
            status: ServerStatus::Offline,
            name,
            category,
            container_name
        }
    }

    pub fn getPort(&self) -> u16{
        self.port
    }

    pub fn getStatus(&self) -> ServerStatus{
        self.status
    }

    pub fn getName(&self) -> &String{
        &self.name
    }

    pub fn getCategory(&self) -> u16{
        self.category
    }

    pub fn getContainerName(&self) -> &String{
        &self.container_name
    }

    pub fn setStatus(&mut self, status: ServerStatus){
        self.status = status;
    }
}