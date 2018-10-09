let player_mod = {
    currnum : -1
}


player_mod.newPlayer = function(name,par_div,mbox){
    let view = document.createElement("div");
    player_mod.currnum ++;
    view.classList.add("player_box","player_"+player_mod.currnum);

    let namebox = document.createElement("div");
    namebox.innerHTML = "<h2>"+name+"</h2>";
    view.appendChild(namebox);

    namebox.onclick = function(){
        view.classList.toggle("player_box");
        view.classList.toggle("hidden_player");
    }

    let res = {
        name:name,
        cards:[],
        view:view,
    };

    let select_card = c => ()=>{ //returns function
        c.selected = ! c.selected ;
        if (c.selected) {
            c.view.classList.add("selected"); 
            mbox.selectCard(c.card,name,true);
        }else {
            c.view.classList.remove("selected");
            mbox.deselectCard(c.card);
        }
    }

    res.addCard = function(c) {
        let crd = card_mod.newCard(c,rooter.card_set);
        crd.view.onclick = select_card(crd);

        this.cards.push(crd);
        this.view.appendChild(crd.view);
    };

    res.getSelected = function(){
        let fres = [];
        for (p in this.cards){
            if (this.cards[p].selected){
                fres.push(this.cards[p]);
            }
        }
        return fres;
    }

    res.dropCard = function(c){
        console.log("DROPPING CARD", c);
        for (p in res.cards){
            let rc = res.cards[p];
            if (rc.card.name !== c.name )continue;
            if (rc.card.kind !== c.kind )continue;
            
            mbox.countDrop(c.kind,res.name);

            rc.view.parentNode.removeChild(rc.view);
            res.cards.splice(p,1);
            return;
        }
    }


    par_div.appendChild(view);

    return res;
}
