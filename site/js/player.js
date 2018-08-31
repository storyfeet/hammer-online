let player_mod = {};


player_mod.new = function(name,par_div){
    let view = document.createElement("div");
    view.classList.add("player_box");
    view.appendChild(document.createTextNode(name));

    res = {
        name:name,
        cards:[],
        view:view,
    };

    res.addCard = function(c){
        cview = document.createElement("div");
        cview.appendChild(document.createTextNode(c.name));
        cview.classList.add("card_box","kind_" + c.kind);

        this.cards.push({card:c,view:cview});
        this.view.appendChild(cview);
    };

    par_div.appendChild(view);

    return res;
}
