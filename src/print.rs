use crate::Options;
use crate::tree::FileNode;
use crate::tree::FileTree;



impl FileNode {
    pub fn print (&self, options: Options, prefix: &mut Vec<String>) -> () {
        let mut depth_limit = false;
        let mut depth = 0;

        // THIS IS FUCKING RETARDED AND UGLY
        if let Some(depth_) = options.depth {
            depth_limit = true;
            if depth_ <= 0 {
                return ();
            }
            depth = depth_;
        }

        for tab in prefix.iter(){
            print!("{}", tab);
        }
        if let Some((_, filename)) = self.name.rsplit_once('/'){
            println!("{}    {}", filename, self.size);
        }
        // println!("{}    {}", self.name, self.size);
        for child in self.children.iter() {

            let mut options_next = options.clone();
            if depth_limit {
                options_next.depth = Some(depth -  1);
            }

            prefix.push("   ".to_string());
            child.print(options_next, prefix);
            prefix.pop();
        }
    }
}


impl FileTree {

}
