$(() => {
    ajax("get", "/api/announcement", undefined, () => $("#announcement")[0].innerHTML = "数据库错误", dat => {
        $("#announcement")[0].innerHTML = "";

        let titleTemplate = Handlebars.compile(`
                <div class="container polygen-item">
                    <a href="javascript: void(0);" data-bs-toggle="modal" data-bs-target="#modal-{{id}}">{{title}}</a>
                </div>`);

        let modalTemplate = Handlebars.compile(`
                <div class="modal fade" id="modal-{{id}}">
                  <div class="modal-dialog modal-xl">
                    <div class="modal-content">
                
                      <div class="modal-header">
                        <h4 class="modal-title">{{title}}</h4>
                        <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                      </div>
                
                      <div class="modal-body"></div>
                
                      <div class="modal-footer">
                        <button type="button" class="btn btn-danger" data-bs-dismiss="modal">关闭</button>
                      </div>
                
                    </div>
                  </div>
                </div>
            `);

        for (let i in dat) {
            let title = dat[i][0], content = dat[i][1];
            $("#announcement").append(titleTemplate({ id: i, title: title }));
            $("#sLeft").append(modalTemplate({ id: i, title: title }));
            $(".modal-body").last()[0].innerHTML = markdownRenderer(content);
        }
    });

    let postEditor = new MarkdownPalettes("#post-editor");

    $("#post").click(() => {
        if (postEditor.content) {
            ajax("post", "/api/post", postEditor.content, msg => swal("发送失败", msg, "error"), () => window.location.href = '/')
        } else {
            swal("发送失败", "内容不能为空", "error");
        }
    });

    let page = 1;
    let postTemplate = Handlebars.compile(`
        <article class="polygen-item">
            <header class="comment-hd">
                {{author}} <span style="color: grey;">{{time}}</span>
            </header>

            <div class="post-content">
                {{content}}
            </div>
        </article>`);

    function addPost() {
        ajax("get", `/api/post?page=${page}`, undefined, undefined, dat => {
            for (let i of dat) {
                $("#main-part").append(postTemplate({ author: i[0], time: i[1] }));
                $(".post-content").last()[0].innerHTML = markdownRenderer(i[2]);
            }
        })
    }

    addPost();
})