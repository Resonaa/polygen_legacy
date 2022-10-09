$(() => {
    $(".ui.sidebar").sidebar("attach events", ".toc.item");

    const vditor = new Vditor("editor", {
        height: 170,
        toolbarConfig: {
            hide: true
        },
        resize: {
            enable: true
        },
        tab: "    ",
        preview: {
            math: {
                inlineDigit: true
            }
        }
    });
});