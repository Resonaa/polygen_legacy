$(() => {
    let pid = Number(window.location.href.match(/\d+$/)[0]);

    let commentEditor = new MarkdownPalettes("#comment-editor");

    $("#comment").click(() => {
        if (commentEditor.content) {
            ajax("post", "/api/comment", { pid: pid, content: commentEditor.content }, msg => swal("发送失败", msg, "error"), () => window.location.reload())
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

            <div class="post-content needs-render" style="max-height: none;overflow-y: hidden;"></div>
        </article>`);

    let dat = ajaxSync("get", `/api/post?pid=${pid}`, undefined).msg;
    $("#main-part").prepend(postTemplate({ realTime: dat.time, deltaTime: deltaTime(dat.time) }));
    $(".post-content").last().html(textRenderer(dat.content));
    $(".post-author").last().html(userLink(dat.author));
    document.title = `${dat.author}的说说 - polygen`;

    function addComment() {
        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/comment?`, { pid: pid, page: page }, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate({ realTime: i.time, deltaTime: deltaTime(i.time), pid: i.pid }));
                $(".post-content").last().html(textRenderer(i.content));
                $(".post-author").last().html(userLink(i.author));
            }
            if (dat.length < 10) {
                $("#load-more").html("没有更多了").click(() => { });
            } else {
                $(".loader").html("点击查看更多...");
            }
        });
    }

    addComment();

    $("#load-more").click(() => {
        page++;
        addComment();
    });

    setInterval(() => $("time").each((_, e) => e.innerHTML = deltaTime(e.title)), 1000);
});