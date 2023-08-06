mod player;
mod coordinate;
mod board;


extern crate yew;
extern crate rand;



use rand::Rng;
use yew::html::Scope;
use yew::{classes, html, Component, Context, Html, Callback};


pub enum Msg {
    Tick,
    MakeMove(usize)
}


pub struct App {
    board: Vec<Option<usize>>,
    cellules_width: usize,
    cellules_height: usize
}


impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback: Callback<()> = ctx.link().callback(|_| Msg::Tick);
        //let interval = Interval::new(200, move || callback.emit(()));
        let (cellules_width, cellules_height) = (3, 3);
        let board = vec![None; cellules_height*cellules_width];

        Self {
            board,
            cellules_width,
            cellules_height
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeMove(idx) => {
                
                self.board[idx] = match self.board[idx] {
                    None => Some(1),
                    _ => None,
                };

                true
            },
            Msg::Tick => true
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cells = vec![1; self.cellules_height*self.cellules_width];
        let cell_rows = self.board
                .chunks(self.cellules_width)
                .enumerate()
                .map(|(row, cellules)| {
                    let idx_offset = row * self.cellules_width;
                    let cells = cellules
                        .iter()
                        .enumerate()
                        .map(|(col, cell)| {
                            let class = match cell {
                                None => "cellule-live",
                                _ => "cellule-dead"
                            };
                            html! {
                                <div key={idx_offset+col} class={classes!("game-cellule", class)}
                                    onclick={ctx.link().callback(move |_| Msg::MakeMove(idx_offset+col))}>
                                 </div>
                            }
                        });
                    html! {
                        <div key={row} class="game-row">
                            { for cells }
                        </div>
                    }
                });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <h1 class="app-title">{ "Rust Tac Toe" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for cell_rows }
                        </div>
                        <div class="game-buttons">
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}


fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}