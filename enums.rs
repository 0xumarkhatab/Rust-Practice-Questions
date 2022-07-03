
use crate::reader::{read_int};
enum download_state{
    active,
    paused,
    stopped,
    completed,
}
impl download_state{
    fn get_state(&self)->String{
        match self{
            download_state::active => String::from("active"),
            download_state::paused => String::from("paused"),
            download_state::stopped => String::from("stopped"),
            download_state::completed => String::from("completed"),
            _=> String::from("Undefined"),
            
        }
    }
}


struct File{
    name:String,
    state:download_state,
}

impl File{
    fn print(&self){
        print!("\n\t\t -- File Details -- \n");
        print!("Name := {}\nDownload State := {}\n",self.name,self.state.get_state());
    }
    fn updateState( &mut self,state_type:i32){
        match state_type{
            1=>self.state=download_state::active,
            2=>self.state= download_state::paused,
            3=>self.state = download_state::stopped,
            4=>self.state =download_state::completed,
            _=>(),
        }

        
    }
}

pub fn main(){
    println!("\nDownloading Feature Using ENums and Structs !\n");
    let mut media_player=File{
        name:String::from("VLC Media Player"),
        state:download_state::active,
    };
    
    println!("\nThe File is Downloading !\n State is as Follows \n");
    media_player.print();

    let mut state_option:i32=0;
    while state_option!=5 {
        println!("\n\t\tFile State Change Menu !");
        println!("1 -> Active(Resume) \n2 -> Pause \n3 -> Stop \n4 -> Complete\n5 -> Exit ");
        state_option = read_int();
        const EXIT_CHOICE:i32 = 5;
        if  state_option>EXIT_CHOICE || state_option<0 {
            println!("Whoops ! Invalid Choice !\n");
        }
        else if state_option == EXIT_CHOICE {
                println!("\nThank you for Using the Browser :) \nExitting !");
        }
        else{
            media_player.updateState(state_option);
            print!("\t->>State Updated \n");
            media_player.print();
            
        }
    }




}