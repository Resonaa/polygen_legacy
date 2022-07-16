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
            let title = dat[i].title, content = dat[i].content;
            $("#announcement").append(titleTemplate({ id: i, title: title }));
            $("#sLeft").append(modalTemplate({ id: i, title: title }));
            $(".modal-body").last().html(textRenderer(content));
        }
    });
});