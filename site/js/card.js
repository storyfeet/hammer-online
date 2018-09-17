card_mod = {};

card_mod.newCard = function(c,cset){
    let cview = document.createElement("div");
    cview.appendChild(document.createTextNode(c.name));
    cview.classList.add("card_box","kind_" + c.kind);

    let res = {card:c,view:cview};

    let cdata = cset[c.name + "," + c.kind];
    //console.log("Card Adding",cdata);
    if (cdata.bskill) {
        //console.log("Skill Found");
        let skbox= document.createElement("div");
        skbox.appendChild(document.createTextNode(cdata.bskill));
        skbox.classList.add("kind_skill");
        cview.appendChild(skbox);
    }
    if (cdata.bskill) {
        //console.log("Trait Found");
        let skbox= document.createElement("div");
        skbox.appendChild(document.createTextNode(cdata.btrait));
        skbox.classList.add("kind_trait");
        cview.appendChild(skbox);
    }
    return res;
}

