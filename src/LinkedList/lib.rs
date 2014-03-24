#[desc = "linked list Rust package."];
#[license = "MIT"];

struct Node<T> {
	value: T,
	next: NextNodeType<T>
}

pub enum NextNodeType<T> {
	Null,
	Next(~Node<T>)
}

pub struct List<T> {
	head: NextNodeType<T>
}

fn PushBack_Internal<T>(n: &mut Node<T>, v: T) -> () {
	match (n.next) {
		Null => n.next = Next (~Node {value: v, next: Null} ),
		Next (~ref mut nn) => PushBack_Internal(nn, v)
	}
}

pub fn PushBack<T>(l: &mut List<T>, v: T) -> () {
	match (l.head) {
		Null => l.head = Next (~Node {value: v, next: Null} ),
		Next (~ref mut nn) => PushBack_Internal(nn, v)
	}
}

fn Print_Internal<T: ::std::fmt::Default> (n: &Node<T>) -> () {
	println!("{}", n.value);
	match (n.next) {
		Null => (),
		Next (~ref nn) => Print_Internal(nn)
	}
}

pub fn Print<T: ::std::fmt::Default>(l: &List<T>) -> () {
	match (l.head) {
		Null => (),
		Next (~ref nn) => Print_Internal(nn)
	}
}

fn Pop_Internal<T>(n: &mut Node<T>) -> () {
	let mut remove = false;
	match (n.next) {
		Null => (),
		Next (~ref mut nn) =>
		{
			match (nn.next) {
				Null => remove = true,
				_ => Pop_Internal(nn)
			}
		}
	}

	if remove == true {
		n.next= Null;
	}
		
}

pub fn Pop<T>(l: &mut List<T>) -> () {
	let mut remove = false;
	match (l.head) {
		Null => (),
		Next (~ref mut nn) =>
		{
			match (nn.next) {
				Null => remove = true,
				_ => Pop_Internal(nn)
			}
		}
	}

	if remove == true {
		l.head = Null;
	}
}
