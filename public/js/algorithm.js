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

function markdownRenderer(s) {
    return marked(s.trim()).replace(/(\n)*$/, "");
}

function userLink(username) {
    return `<a href="/user/${username}" class="at">${username}</a>`;
}

function addAt(input) {
    return input.replace(/@([\u4e00-\u9fa5_a-zA-Z0-9]{3,16})/g, `@${userLink("$1")}`);
}
