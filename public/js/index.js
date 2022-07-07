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
                
                      <div class="modal-body">
                        {{content}}
                      </div>
                
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
            $("#sLeft").append(modalTemplate({ id: i, title: title, content: marked(content) }));
        }
    });

    let postEditor = new ModalEditor("post", "发布说说");

    $("#post").click(() =>
        postEditor.show(dat =>
            ajax("post", "/api/post", dat, msg => swal("发送失败", msg, "error"), () => window.location.href = '/')
        )
    );

    let page = 1;
    let postTemplate = Handlebars.compile(`
        <article class="container polygen-item">
            <header class="comment-hd">
                {{author}} <span style="color: grey;">{{time}}</span>
            </header>

            <div>
                {{content}}
            </div>
        </article>`);

    function addPost() {
        ajax("get", `/api/post?page=${page}`, undefined, undefined, dat => {
            for (let i of dat) {
                $("#main-part").append(postTemplate({ author: i[0], time: i[1], content: marked(i[2]) }));
            }
        })
    }

    addPost();
})