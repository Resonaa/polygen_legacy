$(() => {
    ajax("get", "/api/announcement", undefined, () => $("#announcement").html("数据库错误"), dat => {
        $("#announcement").html("");

        let titleTemplate = Handlebars.compile(`
                <article class="announcement">
                    <a class="no-href" data-bs-toggle="modal" data-bs-target="#modal-{{id}}">{{title}}</a>
                </article>`);

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
            $(".modal-body").last().html(textRenderer(content));
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
        <article class="post">
            <header class="post-hd">
                <div class="post-meta">
                    <div class="post-author"></div>
                    <span class="post-time">&nbsp;<time title="{{realTime}}">{{deltaTime}}</time></span>
                </div>
            </header>

            <div class="post-content"></div>
        </article>`);

    function addPost() {
        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/post?page=${page}`, undefined, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate({ realTime: i[1], deltaTime: deltaTime(i[1]) }));
                $(".post-content").last().html(addAt(textRenderer(i[2])));
                $(".post-author").last().html(userLink(i[0]));
            }

            $(".loader").html("点击查看更多...");
        });
    }

    addPost();

    $("#load-more").click(() => {
        page++;
        addPost();
    });

    setInterval(() => $("time").each((_, e) => e.innerHTML = deltaTime(e.title)), 1000);
});