let player_mod = {
    currnum : -1
}


player_mod.newPlayer = function(name,par_div,rooter){
    let view = document.createElement("div");
    player_mod.currnum ++;
    view.classList.add("player_box","player_"+player_mod.currnum);

    let namebox = document.createElement("h2");
    namebox.innerHTML = name;
    view.appendChild(namebox);

    let mbox = document.createElement("div");
    mbox.classList.add("message_box");
    view.appendChild(mbox);

    let res = {
        name:name,
        cards:[],
        view:view,
        toDrop:["Role","Goal","Skill","Trait"],
        mbox : mbox,
    };

    let select_card = c => ()=>{ //returns function
        c.selected = ! c.selected ;
        if (c.selected) {
            c.view.classList.add("selected"); 
        }else {
            c.view.classList.remove("selected");
        }
        res.hint();
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
    res.showMessage = function(m){
        res.mbox.innerHTML = m;
    }

    res.dropCard = function(c){
        for (p in res.cards){
            let rc = res.cards[p];
            if (rc.card.name !== c.name )continue;
            if (rc.card.kind !== c.kind )continue;
            
            let dex = res.toDrop.indexOf(c.kind);
            if (dex !== -1)res.toDrop.splice(dex,1);

            rc.view.parentNode.removeChild(rc.view);
            res.cards.splice(p,1);
            return;
        }
    }

    let drops_selected = function(){
        let ds = res.toDrop.slice();
        let sel = res.getSelected();
        if (sel.length === 0 ) return false;
        for (i in sel){
            let dex = ds.indexOf(sel[i].card.kind);
            if (dex === -1) {
                return false;
            }
            ds.splice(dex,1);
        }
        return true;
    }

    res.hint = function(c){
        while (mbox.hasChildNodes()) mbox.removeChild(mbox.lastChild);
        //try hint ideas in preference order
        if (this.toDrop.length >0 ){ //drop cards

            if (drops_selected()){
                dbutt = document.createElement("button");
                dbutt.innerHTML = "Drop Cards";
                dbutt.onclick = ()=> {
                    rooter.drop_cards(res.getSelected().map(r=>r.card));
                }
                mbox.appendChild(dbutt);
                return;
            }
            
            let mess = "Select one of each Kind to drop: ";
            for (p in this.toDrop){
                mess += this.toDrop[p] + ", ";
            }
            this.showMessage(mess);
            return;

        }//drop

        this.showMessage("What would you like to do?");

    }

    par_div.appendChild(view);

    return res;
}
