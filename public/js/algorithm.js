function ajax(type, url, data, error, success) {
    $.ajax({
        type: type,
        url: url,
        data: JSON.stringify(data),
        dataType: "json",
        success: res => {
            if (res.status == "success") {
                success(res.msg);
            } else {
                error(res.msg);
            }
        }
    });
}

function toast(status, heading, text, time = 1000, afterHidden = () => { }) {
    $.toast({
        text: text,
        heading: heading,
        icon: status,
        showHideTransition: 'slide',
        hideAfter: time,
        position: 'bottom-right',
        afterHidden: afterHidden
    });
}

class ModalEditor {
    constructor(id, title, content = "", value = "") {

        let modalTemplate = Handlebars.compile(`
                <div class="modal fade" id="modal-{{id}}">
                  <div class="modal-dialog modal-xl">
                    <div class="modal-content">
                      <div class="modal-header">
                        <h4 class="modal-title">{{title}}</h4>
                        <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
                      </div>
                
                      <div class="modal-body">
                        <div class="container">
                            <span>{{content}}</span>
                        </div>

                        <div id="editor-container" style="height: 300px">
                            <div id="editor-{{id}}"></div>
                        </div>
                      </div>
                
                      <div class="modal-footer">
                        <button type="button" class="btn btn-danger" data-bs-dismiss="modal">关闭</button>
                        <button type="button" id="submit-{{id}}" class="btn btn-primary">发布</button>
                      </div>
                
                    </div>
                  </div>
                </div>
            `);

        $("#main").append(modalTemplate({ id: id, title: title, content: content }));
        this.id = id;
        this.editor = new MarkdownPalettes(`#editor-${id}`);
        this.editor.content = value;
    }

    show(callback) {
        $(`#submit-${this.id}`).click(() => {
            if (callback(this.editor.content) == 0) {
                $(`#modal-${this.id}`).modal("hide");
            }
        });

        $(`#modal-${this.id}`).modal("show");
    }
}

function markdownRenderer(s) {
    return marked(s.trim()).replace(/(\n)*$/, "");
}
