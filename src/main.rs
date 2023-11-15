extern crate i3ipc;

use std::{process::Command, collections::HashMap};

use i3ipc::{I3Connection, I3EventListener, Subscription, event::{Event, BindingEventInfo}, reply::Workspace};

fn main() {

    let mut connection = I3Connection::connect().unwrap();
    println!("Established connection to i3 (version {})", connection.get_version().unwrap().major);
    let mut listener = I3EventListener::connect().unwrap();
    println!("Connected to event listener");

    listener.subscribe(&[Subscription::Binding]).unwrap();

    for event in listener.listen() {
        println!("{:?}", event);
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

fn update_eww_workspaces(mut workspace: i32) {
    let mut final_text = String::from("");
    let mut bar = '2';
    println!("Workspace was: {}", workspace);
    if workspace > i32::from(9) {
        workspace -= 10; 
        bar = '1';
    }
    println!("Workspace is: {}", workspace);
    println!("Bar is: {}", bar);
    for i in 1..6 {
        if i32::from(i) == workspace {
            final_text.push('󰄯');
        } else {
            final_text.push('󰄰');
        }
        if i != 5 {
            final_text.push_str("  ");
        }
    }
    println!("{}", final_text);
    let mut command_string = String::from("eww update workspaces_");
    command_string.push(bar);
    command_string.push('=');
    command_string.push('"');
    command_string.push_str(final_text.as_str());
    command_string.push('"');
    println!("{}", command_string);
    Command::new("sh").arg("-c").arg(command_string).output().ok();
}

fn parse_workspace_monitor(connection: &mut I3Connection, binding: &BindingEventInfo) {
    let workspace_keys: HashMap<String, i32> = HashMap::from([
        ("a".to_string(), 1),
        ("r".to_string(), 2),
        ("s".to_string(), 3),
        ("t".to_string(), 4),
        ("d".to_string(), 5)
    ]);
    let key:String = binding.binding.symbol.as_ref().unwrap().to_string();
    let mut requested_space = workspace_keys.get(&key).unwrap().to_owned();
    let mods:&Vec<String> = &binding.binding.event_state_mask;

    let workspaces = connection.get_workspaces().unwrap().workspaces;
    let active_monitor = &workspaces.iter().filter(|workspace| workspace.focused == true).next().unwrap().output;
    let workspaces_on_monitor: Vec<&Workspace> = workspaces.iter().filter(|workspace| workspace.output == active_monitor.to_owned()).collect();
    let workspace_exists = workspaces.iter().any(|workspace| workspace.name == requested_space.to_string().to_owned());
    let workspace_is_on_monitor = workspaces_on_monitor.iter().any(|workspace| workspace.name == requested_space.to_string().to_owned());
    let mut command:String;

    println!("{:?}", &binding.binding);
    if mods.contains(&String::from("shift")) {
        command = String::from("move container to workspace ");
    } else {
        println!("User requested to switch to workspace {} on monitor {}", key, active_monitor);
        command = String::from("workspace ");
    }

    println!("Workspaces on {}: {:?}", active_monitor, workspaces_on_monitor);

    if workspace_exists && workspace_is_on_monitor {
        println!("Workspace exists and is on current monitor");
        command.push_str(&requested_space.to_string());
    } else if workspace_exists {
        println!("Workspace exists on another monitor");
        requested_space += 10;
        command.push_str(requested_space.to_string().as_str());
    } else if workspaces_on_monitor.iter().next().unwrap().name.parse::<i32>().unwrap() >= 10 {
        println!("Workspace does not exist, adding to second monitor");
        requested_space += 10;
        command.push_str(requested_space.to_string().as_str());
    } else {
        println!("Workspace does not exist, adding to first monitor");
        command.push_str(&requested_space.to_string());
    }


    let final_command = &command[..];

    println!("Command: {}", final_command);
    let result = connection.run_command(final_command);
    println!("Result: {:?}", result);

    if result.is_ok() && !mods.contains(&String::from("shift")) {
        update_eww_workspaces(requested_space);
    }
}
