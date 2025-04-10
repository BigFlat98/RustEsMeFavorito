

//실습

pub mod task_mod{
    pub struct TaskList{
        pub tasks: Vec<Task>
    
    }
    impl TaskList{
        pub fn new() -> Self{
            TaskList{
                tasks: Vec::new(),
            }
        }
        pub fn add_task(&mut self,task: Task){
            // ai 리팩토링. 중복 체크
            if &self.tasks.iter().any(|t| t.task_name == task.task_name){//
                Err("Task already exists".to_string())
            }
            else{
                self.tasks.push(task);
            }
        }
        // ai 리팩토링. 완료 체크
        pub fn complete_task(&mut self, title: &str) -> Result<(), String> {
            let task = self.tasks
                .iter_mut()
                .find(|t| t.task_name == title)
                .ok_or("할 일을 찾을 수 없습니다.")?;  // ? 연산자 사용
            
            task.completed = true;
            Ok(())
        }
        pub fn get_task(&self,task_name: String) -> Option<&Task>{
            self.tasks.iter().find(|task| task.task_name == task_name) //find메서드의 |n|은 파라미터 값인 것 같음
        }
        pub fn get_all_tasks(&self) -> Result<&Vec<Task>,String>{
            if self.tasks.is_empty() {//난 여기서 len메서드를 사용하려 했는데 배열은 그냥 is_empty() 사용하면 bool로 리턴된
                Err("No tasks".to_string())
            }
            else{
                for task in &self.tasks{
                    task.task_info();
                }
                Ok(&self.tasks) //Ok로 반환할 때는 소유권 생각해서 clone을 넘겨주나봄.
            }
        }
        
    }
    
    
    
    pub struct Task{
        task_name: String,
        completed: bool,
        content: String,
        start_date: Option<String>,
        end_date: Option<String>,
    }
    impl Task{
        pub fn new(task_name: String, content: String) -> Self{
            Task{
                task_name,
                completed: false,
                content,
                start_date: None,
                end_date: None,
            }
        }
        pub fn start(&mut self,start_d: String){
            self.start_date = Some(start_d);
        }
        pub fn end(&mut self,end_d: String){
            self.end_date = Some(end_d);
        }
        pub fn modify_content(&mut self,content: String){
            self.content = content;
        }
        pub fn done(&mut self){
            self.completed = true;
        }
        pub fn undone(&mut self){
            self.completed = false;
        }
        pub fn task_info(&self){
            println!("Task Name: {}",self.task_name);
            println!("Completed: {}",self.completed);
            println!("Content: {}",self.content);
            match &self.start_date{
                Some(start_date) => println!("Start Date: {}",start_date),
                None => println!("Not started"),
            }
            match &self.end_date{
                Some(end_date) => println!("End Date: {}",end_date),
                None => println!("Not finished"),
            }
            
        }
        
    } 
}
