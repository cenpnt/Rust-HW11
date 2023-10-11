use std::fs;
use std::fs::File;
use std::io::Write;
fn main() {
    // let fox = "The quick brown fox jumps over the lazy dog.";
    // let para3 = "a\n\nb\n\nc";
    // let bustle = "\
    //     The bustle in a house\n\
    //     The morning after death\n\
    //     Is solemnest of industries\n\
    //     Enacted upon earth,—\n\
    //     \n\
    //     The sweeping up the heart,\n\
    //     And putting love away\n\
    //     We shall not want to use again\n\
    //     Until eternity.\n\
    //     ";

    // let doc1 = make_document(fox); // 1 paragraph
    // let doc2 = make_document(bustle); // 2 paragraphs
    // let doc3 = make_document(para3);

    // println!("{:?} {:?} {:?}", doc1, doc2, doc3);
    // let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    // println!("{:?}", docs);
    gen_html();
    gen_html_count();
}

fn make_document(text: &str) -> Vec<String> {
    text.split("\n\n").map(|paragraph| paragraph.to_string()).collect() // split \n\n and map to string then collect to vector
}

fn rank_documents(docs: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut rnk_docs = docs.to_vec(); //convert slice to vector
    rnk_docs.sort_by(|a, b| b.len().cmp(&a.len())); // descending order 3 para 2 para 1 para
    rnk_docs // return
}


#[test]
fn test_doc() {
    let fox = "The quick brown fox jumps over the lazy dog.";
    let para3 = "a\n\nb\n\nc";
    let bustle = "\
        The bustle in a house\n\
        The morning after death\n\
        Is solemnest of industries\n\
        Enacted upon earth,
        —\n\
        \n\
        The sweeping up the heart,\n\
        And putting love away\n\
        We shall not want to use again\n\
        Until eternity.\n\
        ";
    let doc1 = make_document(fox); // 1 paragraph
    let doc2 = make_document(bustle); // 2 paragraphs
    let doc3 = make_document(para3); // 3 paragraphs

    let docs = vec![doc1.clone(), doc3.clone(), doc2.clone()];
    let rnk_docs = rank_documents(&docs);

    assert_eq!(rnk_docs, [doc3, doc2, doc1]);
}

fn read_doc(file: &str) -> Vec<String> {
    let text = fs::read_to_string(format!("src/{}", file)).unwrap();
    make_document(&text)
}

fn gen_html() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>HTML Table</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 25%;
                border-collapse: collapse;
                margin: 20px;
                }
            </style>
            <h1>Report number of paragraphs</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>File name</th>
                        <th>Number of paragraphs</th>
                    </tr>
                </thead>
                <tbody>"); 

    let files = vec!["fox.txt", "bustle.txt", "para3.txt", "empty.txt"];

    let mut list_doc = Vec::new();
    for i in 0..files.len() {
        let doc = read_doc(files[i]);
        list_doc.push(doc)
    }

    let rnk_doc = rank_documents(&list_doc);
    // println!("{:?}", rnk_doc);

    for doc in rnk_doc {
        for file in &files {
            if read_doc(file) == doc {
                if file == &"empty.txt" {
                    table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", file, 0));
                    continue;
                } 
                table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", file, doc.len()));
            }
            else {
                continue;
            }
        }
    }

    table.push_str("</tbody></table></body></html>");

    let mut file = File::create("paragraph.html").unwrap();
    file.write(table.as_bytes()).unwrap();

}

fn gen_html_count() {
    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>HTML Table</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 25%;
                border-collapse: collapse;
                margin: 20px;
                }
            </style>
            <h1>Report word counts</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>File name</th>
                        <th>Word Counts</th>
                    </tr>
                </thead>
                <tbody>"); 

    let files = vec!["fox.txt", "bustle.txt", "para3.txt", "empty.txt"];

    let mut list_doc = Vec::new();
    for i in 0..files.len() {
        let doc = read_doc(files[i]);
        list_doc.push(doc)
    }

    let rank_doc = rank_count(&list_doc);

    for doc in rank_doc {
        for file in &files {
            if read_doc(file) == doc {
                table.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", file, count_words(doc.clone())));
            }
            else {
                continue;
            }
        }
    }


    table.push_str("</tbody></table></body></html>");

    let mut file = File::create("count.html").unwrap();
    file.write(table.as_bytes()).unwrap();

}

fn count_words(docs: Vec<String>) -> usize {
    let mut total = 0;
    for doc in docs {
        let split = doc.split_whitespace();
        total += split.count();
    }
    total
}

fn rank_count(docs: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut rnk_count = docs.to_vec();
    rnk_count.sort_by(|a, b| count_words(b.to_vec()).cmp(&count_words(a.to_vec())));
    rnk_count
}