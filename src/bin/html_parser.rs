
// html2text = "0.5.1"
use html2text::from_read;


fn main() {
    // 你的 HTML 字符串
    let html_content = r#"
        <html>
        <head>
            <title>示例页面</title>
        </head>
        <body>你好世界
            <h1>欢迎来到示例页面</h1>
            <p>这是一个用于演示的段落。</p>
            <p>这是另一个段落。</p>
            <div class="container">
                <p>这是一个包含在 div 容器中的段落。</p>
                <div>
                最后一段内容
                </div>
            </div>
        </body>
        </html>
    "#;

    let text = from_read(html_content.as_bytes(), 6000);
    println!("{}", text);
}
