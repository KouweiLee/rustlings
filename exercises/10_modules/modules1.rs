// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
// 一般文件中.rs就是一个mod，mod的pub和非pub就类似于C文件中的not static和static