// Lesson 06.3: Practical Example: File-based Outbox Simulation

// This lesson combines what we have learned about file I/O and error handling
// to create a simple simulation of an outbox pattern. In this pattern, a service
// writes events to an "outbox" (in this case, a file), and another service
// reads from the outbox and processes the events.

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

// --- The Event ---

// First, let's define the event that we want to write to the outbox.

#[derive(Debug)]
struct Event {
    id: u64,
    payload: String,
}

impl Event {
    fn new(id: u64, payload: &str) -> Self {
        Event { id, payload: payload.to_string() }
    }

    fn to_string(&self) -> String {
        format!("{}:{}", self.id, self.payload)
    }

    fn from_string(s: &str) -> Result<Self, &'static str> {
        let mut parts = s.splitn(2, ':');
        let id_str = parts.next().ok_or("Missing id")?;
        let payload = parts.next().ok_or("Missing payload")?;

        let id = id_str.parse().map_err(|_| "Invalid id")?;

        Ok(Event { id, payload: payload.to_string() })
    }
}

// --- The Outbox ---

// The outbox is responsible for writing events to a file.

struct Outbox {
    file_path: String,
}

impl Outbox {
    fn new(file_path: &str) -> Self {
        Outbox { file_path: file_path.to_string() }
    }

    fn write_event(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "{}", event.to_string())
    }
}

// --- The Event Processor ---

// The event processor is responsible for reading events from the outbox and
// processing them.

struct EventProcessor {
    file_path: String,
}

impl EventProcessor {
    fn new(file_path: &str) -> Self {
        EventProcessor { file_path: file_path.to_string() }
    }

    fn process_events(&self) -> io::Result<()> {
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            match Event::from_string(&line) {
                Ok(event) => {
                    println!("Processing event: {:?}", event);
                }
                Err(e) => {
                    eprintln!("Error parsing event: {}", e);
                }
            }
        }

        // In a real application, we would probably want to delete the file or
        // move it to an archive after processing.
        fs::remove_file(&self.file_path)?;

        Ok(())
    }
}

fn main() -> io::Result<()> {
    let outbox_file = "outbox.txt";

    // --- Write some events to the outbox ---
    let outbox = Outbox::new(outbox_file);
    outbox.write_event(&Event::new(1, "User created"))?;
    outbox.write_event(&Event::new(2, "User updated"))?;
    outbox.write_event(&Event::new(3, "User deleted"))?;

    // --- Process the events ---
    let processor = EventProcessor::new(outbox_file);
    processor.process_events()?;

    Ok(())
}
