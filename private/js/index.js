$(() => {
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
                    <span class="post-time">&nbsp;<time title="{{realTime}}">{{deltaTime}}</time>&nbsp;{{commentAmount}}评论</span>
                </div>
            </header>

            <a href="/post/{{pid}}" style="color: unset;"><div class="post-content needs-render"></div></a>
        </article>`);

    function addPost() {
        $(".loader").html(`<div class="spinner-border"></div>`);

        ajax("get", `/api/post?`, { page: page }, undefined, dat => {
            for (let i of dat) {
                $("#load-more").before(postTemplate({ realTime: i.time, deltaTime: deltaTime(i.time), pid: i.pid, commentAmount: ajaxSync("get", "/api/post/commentamount?", { pid: i.pid }).msg }));
                $(".post-content").last().html(i.content);
                $(".post-author").last().html(userLink(i.author));
            }

            if (dat.length < 10) {
                $("#load-more").html("没有更多了").click(() => { });
            } else {
                $(".loader").html("点击查看更多...");
            }

            renderAll();
        });
    }

    addPost();

    $("#load-more").click(() => {
        page++;
        addPost();
    });

    setInterval(() => $("time").each((_, e) => e.innerHTML = deltaTime(e.title)), 1000);
});