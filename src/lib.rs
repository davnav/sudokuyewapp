#![recursion_limit="512"]

use wasm_bindgen::prelude::*;

use cell::Cell;
use yew::prelude::*;
//use yew::services::interval::{IntervalService, IntervalTask};
use yew::{Classes,html,Component,ComponentLink,Html,ShouldRender};
//use yew::virtual_dom::*;
use yew::Properties;

mod cell;
use cell::State;

pub enum Msg{
  //  ToggleCellule(usize),
    Solve,
}
#[derive(Clone,Debug)]
pub struct Cellules{
    link:ComponentLink<Self>,
    cellule:Vec<Cell>,
   // path:Vec<(usize,usize)>,
    // props:Props,

}

impl Cellules{
    fn view_cellule(&self, x:usize,cell:&Cell) -> Html{

 
        let mut cell_color =  match  cell.state {
                State::yellow => { 'y' },
                State::green => { 'g' },
        };

             // log::info!("{:?}",cell_color);
        html!{
            <div key=x class=format!("game-cellule{}",cell_color)>{ cell.value } </div> 
        }

    }

//     /// rat-maze problem solving algorithm wrapper
//     /// this method call easy when clicking
    fn solvewrap(&mut self){
        let row =0;
        let col = 0;
        self.solve(); 
    }

    ///boundary check of the Maze or Grid
    /// this is would be useful in the actual algorithm
    fn getemptycell(&self) -> Option<(usize,usize)>{

        for i in 0..9{
            for j in 0..9{
                if self.cellule[(i*8+i)+j].value == 0{
                        log::info!("{},{}",i,j);
                        log::info!("{:?}",self.cellule);
                        log::info!("{:?},{:?}",self.cellule[(i*8+i)+j].row,self.cellule[(i*8+i)+j].col);
                        return Some((self.cellule[(i*8+i)+j].row,self.cellule[(i*8+i)+j].col))
                }
                
                
              //  log::info!("{},{},{}",self.cellule[(i*8+i)+j].value,self.cellule[(i*8+i)+j].row,self.cellule[(i*8+i)+j].col);
                

            }
        }

        return None
      
    }

    ///Actual method where the backtracking algorithm is implemented
    /// this method will be recursively called based the choices
    /// we will be adding the "path" in this algorithm, before calling the method recursively
    /// if we couldn't solve the Maze, pop the added "path" and next choice we have.
    fn solve(&mut self)-> bool{
        
        if let Some((row,col)) = self.getemptycell(){
                for value in 1..10{
                        //col = col+1;
                        if self.IsValidValue(row,col,value){
                            // let index = (row*8 + row) + col ;
                            self.cellule[(row*8+row)+col].value = value;
                            log::info!("value = {},row={},col={}",value,row,col);
                            if self.solve(){
                                return true

                            }
                            self.cellule[(row*8+row)+col].value = 0;
                        }
                }
                return false
                                // },
                    // None      =>   { 
                    //                 log::info!("{:?}",self.cellule);
                    //                 return true
                    //             },
        }else{
            log::info!("{:?}",self.cellule);
            return true
        } 


    }

    fn IsValidValue(&self,row:usize,col:usize,value:u8) -> bool{
           
        
          let areacheckbool = self.areacheck(row,col,value);
          let colcheckbool = self.colcheck(row,col,value);
          let rowcheckbool = self.rowcheck(row,col,value);

       //   log::info!("areacheck={},colcheck={},rowcheck={}",areacheckbool,colcheckbool,rowcheckbool);
          
          if areacheckbool && colcheckbool && rowcheckbool{
                return true
          }else{
                return false
          }
            

    }


    fn areacheck(&self,row:usize,col:usize,value:u8) -> bool{

        if (0 <= col && col < 3) && (0 <= row && row < 3) {
            for  i in 0..3{
                for j in 0..3{
                    if self.cellule[(i*8 + i) +j].value == value && (j != col || i != row){
                           return false
                    }else {
                        continue;
                    }
                }
            }
            
            return true

        }

        else if (0<= col && col < 3) && ( 3 <= row && row < 6){
                for i in 3..6{
                    for j in 0..3{
                        if self.cellule[(i*8 + i )+j].value == value && (j != col || i != row) {
                            return false
                        }else{
                            continue
                        }
                    }
                }

                return true
        }


        else if (0<= col && col < 3) && ( 6 <= row && row  < 9 ){
                for i in 6..9{
                    for j in 0..3{
                        if self.cellule[(i*8 + i )+j].value == value && (j != col || i != row) {
                            return false
                        }else {
                            continue
                        }
                    }
                }
                return true
        }


        else if (3 <= col && col < 6) && ( 0 <= row && row < 3 ){
                for i in 0..3{
                    for j in 3..6{
                        if self.cellule[(i*8 + i)+j].value == value && (j != col || i != row) {
                            return false
                        }else {
                            continue
                        }
                    }
                    
                }

                return true
        }

        else if (3 <= col && col < 6) && ( 3 <= row && row < 6 ){
            for i in 3..6{
                for j in 3..6{
                    if self.cellule[( i*8 + i )+j].value == value && (j != col || i != row) {
                        return false
                    }else {
                        continue
                    }
                }
                
            }

            return true
        }

        else if (3 <= col && col < 6) && ( 6 <= row && row < 9 ){
            for i in 6..9{
                for j in 3..6{
                    if self.cellule[(i*8+i)+j].value == value && (j != col || i != row) {
                        return false
                    }else {
                        continue
                    }
                }
                
            }

            return true
        }


        else if (6 <= col && col < 9) && ( 0 <= row && row < 3 ){
            for i in 0..3{
                for j in 6..9{
                    if self.cellule[(i*8+i)+j].value == value && (j != col || i != row) {
                        return false
                    }else {
                        continue
                    }
                }
                
            }

            return true
        }

        else if (6 <= col && col < 9) && ( 3 <= row && row < 6 ){
            for i in 3..6{
                for j in 6..9{
                    if self.cellule[(i*8 + i)+j].value == value && (j != col || i != row) {
                        return false
                    }else {
                        continue
                    }
                }
                
            }

            return true
        }

        else if (6 <= col && col < 9) && ( 6 <= row && row < 9 ){
            for i in 6..9{
                for j in 6..9{
                    if self.cellule[(i*8 + i)+j].value == value && (j != col || i != row) {
                        return false
                    }else {
                        continue
                    }
                }
                
            }

            return true
        }else {
            return false 
        }


    }

    fn colcheck(&self, row:usize,col:usize,value:u8) -> bool{

        for i in 0..9{
            if self.cellule[(i*8+i)+col].value == value  && i != row{
                return false
            }else {
                continue
            }
        }

        return true
    }

    fn rowcheck(&self, row:usize,col:usize,value:u8) -> bool{

        for i in 0..9{
            if self.cellule[(row*8+row)+i].value == value  && i != col {
                return false
            }else {
                continue
            }
        }

        return true
    }


     

 }


// #[derive(Clone,Properties)]
// pub struct Props{
//     #[prop_or_default]
//     class:Classes
// }
// impl Default for Color{
//     fn default() -> Self{
//         Color::Yellow
//     }


impl Component for Cellules{

    type Message = Msg;
    type Properties = ();

    fn create(props:Self::Properties,link:ComponentLink<Self>) -> Self{

        Self{
            link,
            cellule:vec![Cell::new_yellow(0,0,0),Cell::new_yellow(0,0,1),Cell::new_yellow(0,0,2),Cell::new_yellow(2,0,3),Cell::new_yellow(6,0,4),Cell::new_yellow(0,0,5),Cell::new_yellow(7,0,6),Cell::new_yellow(0,0,7),Cell::new_yellow(1,0,8),
            Cell::new_yellow(6,1,0),Cell::new_yellow(8,1,1),Cell::new_yellow(0,1,2), Cell::new_yellow(0,1,3),Cell::new_yellow(7,1,4),Cell::new_yellow(0,1,5),Cell::new_yellow(0,1,6),Cell::new_yellow(9,1,7),Cell::new_yellow(0,1,8),
            Cell::new_yellow(1,2,0),Cell::new_yellow(9,2,1),Cell::new_yellow(0,2,2),Cell::new_yellow(0,2,3),Cell::new_yellow(0,2,4),Cell::new_yellow(4,2,5),Cell::new_yellow(5,2,6),Cell::new_yellow(0,2,7),Cell::new_yellow(0,2,8),
            Cell::new_yellow(8,3,0),Cell::new_yellow(2,3,1),Cell::new_yellow(0,3,2),Cell::new_yellow(1,3,3),Cell::new_yellow(0,3,4),Cell::new_yellow(0,3,5),Cell::new_yellow(0,3,6),Cell::new_yellow(4,3,7),Cell::new_yellow(0,3,8),
            Cell::new_yellow(0,4,0),Cell::new_yellow(0,4,1),Cell::new_yellow(4,4,2),Cell::new_yellow(6,4,3),Cell::new_yellow(0,4,4),Cell::new_yellow(2,4,5),Cell::new_yellow(9,4,6),Cell::new_yellow(0,4,7),Cell::new_yellow(0,4,8),
            Cell::new_yellow(0,5,0),Cell::new_yellow(5,5,1),Cell::new_yellow(0,5,2),Cell::new_yellow(0,5,3),Cell::new_yellow(0,5,4),Cell::new_yellow(3,5,5),Cell::new_yellow(0,5,6),Cell::new_yellow(2,5,7),Cell::new_yellow(8,5,8),
            Cell::new_yellow(0,6,0),Cell::new_yellow(0,6,1),Cell::new_yellow(9,6,2),Cell::new_yellow(3,6,3),Cell::new_yellow(0,6,4),Cell::new_yellow(0,6,5),Cell::new_yellow(0,6,6),Cell::new_yellow(7,6,7),Cell::new_yellow(4,6,8),
            Cell::new_yellow(0,7,0),Cell::new_yellow(4,7,1),Cell::new_yellow(0,7,2),Cell::new_yellow(0,7,3),Cell::new_yellow(5,7,4),Cell::new_yellow(0,7,5),Cell::new_yellow(0,7,6),Cell::new_yellow(3,7,7),Cell::new_yellow(6,7,8),
            Cell::new_yellow(7,8,0),Cell::new_yellow(0,8,1),Cell::new_yellow(3,8,2),Cell::new_yellow(0,8,3),Cell::new_yellow(1,8,4),Cell::new_yellow(8,8,5),Cell::new_yellow(0,8,6),Cell::new_yellow(0,8,7),Cell::new_yellow(0,8,8)],
            //  props:props,
          //  props:props,
         //   path:Vec::new(),

        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg{
            Msg::Solve => {
                            self.solvewrap();
                            log::info!("solved");
                        }
        }     
                true 
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
       false 
    }

    

    fn view(&self) -> Html{
            let cell_rows = self.cellule.chunks(9)
                                    .enumerate()
                                    .map(|(y,cell_row)| {

                                        let cells = cell_row.iter()
                                                    .enumerate()
                                                    .map(|(x,cell)| {
                                                        self.view_cellule(x,cell)
                                                    }) ;
                                        html!{
                                                <div >{ for cells } </div>
                                          
                                        }
                                        
                                    });
                                    
            html!{
                <div>
                    <div> { for cell_rows }</div>
                    <button onclick=self.link.callback( |_| Msg::Solve)>{"solve"}</button> 
                </div>
            }                        
                                    
                                    
                                    
                                   

         
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {

    ///starting the Yew app
    yew::start_app::<Cellules>();
    wasm_logger::init(wasm_logger::Config::default());

    log::info!("Click on the solve  button for solving sudoku");
}