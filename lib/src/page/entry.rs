#[derive(Clone, Debug, PartialEq)]
enum EntryKind {
    Note,
    Task(TaskStatus),
}

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    kind: EntryKind,
    tags: HashMap<String, String>,
    children: Vec<Entry>

}

