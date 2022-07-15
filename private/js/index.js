$(() => {
    let pid = 0;
    let s = window.location.href.match(/\d+$/);
    if (s != null) {
        pid = Number(s[0]);
    }

    let postEditor = new MarkdownPalettes("#post-editor");

    $("#post").click(() => {
        if (postEditor.content) {
            ajax("post", "/api/post", { content: postEditor.content, parent: pid }, msg => swal("发送失败", msg, "error"), () => window.location.reload())
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
                    <span class="post-time">&nbsp;<time title="{{realTime}}">{{deltaTime}}</time>&nbsp;{{commentAmount}}评论</span>
                </div>
            </header>

            <a href="/post/{{pid}}" style="color: unset;"><div class="post-content needs-render"></div></a>
        </article>`);

    if (pid != 0) {
        let dat = ajaxSync("get", `/api/post?pid=${pid}`, undefined).msg;
        $("#main-part").prepend(postTemplate({ realTime: dat.time, deltaTime: deltaTime(dat.time), commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: dat.pid }).msg }));
        $(".post-content").last().html(textRenderer(dat.content)).css("max-height", "none").css("overflow-y", "hidden")[0].parentElement.attributes.removeNamedItem("href");
        $(".post-author").last().html(userLink(dat.author));
        document.title = `${dat.author}的说说 - polygen`;
    }

    function addPost() {
        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/post?`, { parent: pid, page: page }, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate({ realTime: i.time, deltaTime: deltaTime(i.time), pid: i.pid, commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: i.pid }).msg }));
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

    addPost();

    $("#load-more").click(() => {
        page++;
        addPost();
    });
});