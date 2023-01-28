extern crate i3ipc;

use i3ipc::{I3Connection, I3EventListener, Subscription, event::{Event, BindingEventInfo}, reply::Workspace};

fn main() {

    let mut connection = I3Connection::connect().unwrap();
    println!("Established connection to i3 (version {})", connection.get_version().unwrap().major);
    let mut listener = I3EventListener::connect().unwrap();
    println!("Connected to event listener");

    listener.subscribe(&[Subscription::Binding]).unwrap();

    for event in listener.listen() {
        match event.unwrap() {
            Event::ModeEvent(_) => todo!(),
            Event::WorkspaceEvent(_) => todo!(),
            Event::OutputEvent(_) => todo!(),
            Event::WindowEvent(_) => todo!(),
            Event::BarConfigEvent(_) => todo!(),
            Event::BindingEvent(e) => if e.binding.command == "nop" {parse_workspace_monitor(&mut connection,  &e)},
            Event::ShutdownEvent(_) => todo!(),
        }
    }
}

fn parse_workspace_monitor(connection: &mut I3Connection, binding: &BindingEventInfo) {
    let key:&String = binding.binding.symbol.as_ref().unwrap();
    let mods:&Vec<String> = &binding.binding.event_state_mask;

    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let active_monitor = &workspaces.iter().filter(|workspace| workspace.focused == true).next().unwrap().output;
    let workspaces_on_monitor: Vec<&Workspace> = workspaces.iter().filter(|workspace| workspace.output == active_monitor.to_owned()).collect();
    let workspace_exists = workspaces.iter().any(|workspace| workspace.name == key.to_owned());
    let workspace_is_on_monitor = workspaces_on_monitor.iter().any(|workspace| workspace.name == key.to_owned());
    let mut command:String;

    println!("{:?}", &binding.binding);
    if mods.contains(&String::from("shift")) {
        command = String::from("move container to workspace ");
    } else {
        command = String::from("workspace ");
    }
    println!("User requested to switch to workspace {} on monitor {}", key, active_monitor);
    println!("Workspaces on {}: {:?}", active_monitor, workspaces_on_monitor);

    if workspace_exists && workspace_is_on_monitor {
        println!("Workspace exists and is on current monitor");
        command.push_str(&key);
    } else if workspace_exists {
        println!("Workspace exists on another monitor");
        let mut requested_space = key.parse::<i32>().unwrap();
        requested_space += 10;
        command.push_str(requested_space.to_string().as_str());
    } else if workspaces_on_monitor.iter().next().unwrap().name.parse::<i32>().unwrap() >= 10 {
        println!("Workspace does not exist, adding to second monitor");
        let mut requested_space = key.parse::<i32>().unwrap();
        requested_space += 10;
        command.push_str(requested_space.to_string().as_str());
    } else {
        println!("Workspace does not exist, adding to first monitor");
        command.push_str(&key);
    }

    let final_command = &command[..];

    println!("Command: {}", final_command);
    let result = connection.run_command(final_command);
    println!("Result: {:?}", result);
}
