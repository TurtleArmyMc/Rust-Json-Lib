pub trait Marshalable {
    fn marshal_json(&self) -> String {
        let mut s = String::new();
        self.marshal_json_into(&mut s);
        s
    }

    fn marshal_json_into(&self, s: &mut String);
}
