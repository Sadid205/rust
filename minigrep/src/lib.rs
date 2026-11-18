pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); // খালি তালিকা

    for line in contents.lines() {
        // ফাইলের প্রতিটি লাইনের জন্য
        if line.contains(query) {
            // যদি লাইনে query থাকে
            results.push(line); // তালিকায় যোগ করো
        }
    }

    results // তালিকাটা ফেরত দাও
}
