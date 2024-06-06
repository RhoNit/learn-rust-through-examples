enum Task {
    Todo(TodoTask),
    Deadline(DeadlineTask),
    Event(EventTask),
}

struct TodoTask {
    description: String,
    priority: Priority,
}

struct DeadlineTask {
    description: String,
    due_date: String,
    priority: Priority,
}

struct EventTask {
    description: String,
    event_date: String,
    priority: Priority,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd)]
enum Priority {
    NotRequired,
    NeedToDeliverItRightAway,
    CanBePutOffToNextRelease,
    MostUrgent,
    Urgent,
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Priority::NeedToDeliverItRightAway, Priority::NeedToDeliverItRightAway) => std::cmp::Ordering::Equal,
            (Priority::NeedToDeliverItRightAway, _) => std::cmp::Ordering::Greater,

            (Priority::MostUrgent, Priority::NeedToDeliverItRightAway) => std::cmp::Ordering::Less,
            (Priority::MostUrgent, Priority::MostUrgent) => std::cmp::Ordering::Equal,
            (Priority::MostUrgent, _) => std::cmp::Ordering::Greater,

            (Priority::Urgent, Priority::NeedToDeliverItRightAway) => std::cmp::Ordering::Less,
            (Priority::Urgent, Priority::MostUrgent) => std::cmp::Ordering::Less,
            (Priority::Urgent, Priority::Urgent) => std::cmp::Ordering::Equal,
            (Priority::Urgent, _) => std::cmp::Ordering::Greater,

            (Priority::CanBePutOffToNextRelease, Priority::CanBePutOffToNextRelease) => std::cmp::Ordering::Equal,
            (Priority::CanBePutOffToNextRelease, Priority::NotRequired) => std::cmp::Ordering::Greater,
            (Priority::CanBePutOffToNextRelease, _) => std::cmp::Ordering::Less,

            (Priority::NotRequired, Priority::NotRequired) => std::cmp::Ordering::Equal,
            (Priority::NotRequired, _) => std::cmp::Ordering::Less,

        }
    }
}

impl Priority {
    fn sort_tasks(tasks: &mut Vec<Task>) {
        tasks.sort_by(|t1, t2| {
            let t1_priority = Task::get_priority(t1);
            let t2_priority = Task::get_priority(t2);

            t2_priority.cmp(&t1_priority)
        });
    }
}

impl Task {
    fn display_task_info(&self) {
        match self {
            Task::Todo(todo) => println!("TODO | {} (priority -> {:?})", todo.description, todo.priority),
            Task::Deadline(deadline) => println!("DEADLINE | {} (due date -> {}, priority -> {:?})", deadline.description, deadline.due_date, deadline.priority),
            Task::Event(event) => println!("EVENT | {} (event date -> {}, priority -> {:?})", event.description, event.event_date, event.priority),
        }
    }

    fn get_priority(task: &Task) -> Priority {
        match task {
            Task::Todo(todo) => todo.priority.clone(),
            Task::Deadline(deadline) => deadline.priority.clone(),
            Task::Event(event) => event.priority.clone(),
        }
    }
}

fn main() {
    let mut tasks_list = vec![
        Task::Todo(TodoTask {
            description: String::from("Todo task"),
            priority: Priority::CanBePutOffToNextRelease,
        }),
        Task::Deadline(DeadlineTask {
            description: String::from("Submit release note for 5.4"),
            due_date: String::from("2024-05-15"),
            priority: Priority::NeedToDeliverItRightAway,
        }),
        Task::Event(EventTask {
            description: String::from("Performance review meeting"),
            event_date: String::from("2024-05-16"),
            priority: Priority::Urgent,
        }),
        Task::Event(EventTask {
            description: String::from("Learning Tuesday Session"),
            event_date: String::from("2024-05-18"),
            priority: Priority::NotRequired,
        }),
        Task::Todo(TodoTask {
            description: String::from("Develop API endpoints for new-feature"),
            priority: Priority::MostUrgent,
        }),
        Task::Todo(TodoTask {
            description: String::from("Design DB for new entities"),
            priority: Priority::NeedToDeliverItRightAway,
        }),
    ];

    println!("Tasks (before sorting): ");
    println!("------------------------------------------------------------------------");
    for t in &tasks_list {
        t.display_task_info();
    }

    Priority::sort_tasks(&mut tasks_list);

    println!("\n\nTasks (after sorting): ");
    println!("------------------------------------------------------------------------");
    for t in &tasks_list {
        t.display_task_info();
    }
}
