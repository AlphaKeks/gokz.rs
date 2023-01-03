version 6.0
let s:cpo_save=&cpo
set cpo&vim
cnoremap <silent> <Plug>(TelescopeFuzzyCommandSearch) e "lua require('telescope.builtin').command_history { default_text = [=[" . escape(getcmdline(), '"') . "]=] }"
noremap! <silent> <Plug>luasnip-expand-repeat <Cmd>lua require'luasnip'.expand_repeat()
noremap! <silent> <Plug>luasnip-delete-check <Cmd>lua require'luasnip'.unlink_current_if_deleted()
inoremap <silent> <Plug>luasnip-jump-prev <Cmd>lua require'luasnip'.jump(-1)
inoremap <silent> <Plug>luasnip-jump-next <Cmd>lua require'luasnip'.jump(1)
inoremap <silent> <Plug>luasnip-prev-choice <Cmd>lua require'luasnip'.change_choice(-1)
inoremap <silent> <Plug>luasnip-next-choice <Cmd>lua require'luasnip'.change_choice(1)
inoremap <silent> <Plug>luasnip-expand-snippet <Cmd>lua require'luasnip'.expand()
inoremap <silent> <Plug>luasnip-expand-or-jump <Cmd>lua require'luasnip'.expand_or_jump()
inoremap <C-W> u
inoremap <C-U> u
tnoremap  <Cmd>wincmd h
nnoremap  <Cmd>wincmd h
tnoremap <NL> <Cmd>wincmd j
nnoremap <NL> <Cmd>wincmd j
tnoremap  <Cmd>wincmd k
nnoremap  <Cmd>wincmd k
tnoremap  <Cmd>wincmd l
nnoremap  <Cmd>wincmd l
nnoremap  <Cmd>w
nnoremap  <Cmd>tabnew<Cmd>termA
tnoremap  <Cmd>tabclose
nnoremap  <Cmd>close
xnoremap  c <Plug>(comment_toggle_linewise_visual)
snoremap  c <Plug>(comment_toggle_blockwise_visual)
nnoremap  c <Plug>(comment_toggle_linewise_current)
nmap <silent>  np :NoNeckPain
tnoremap   
tnoremap  vs <Cmd>vsplit
nnoremap  vs <Cmd>vsplit
tnoremap  ss <Cmd>split
nnoremap  ss <Cmd>split
vnoremap  p "+p
nnoremap  p "+p
vnoremap  Y "+y$
nnoremap  Y "+y$
vnoremap  y "+y
nnoremap  y "+y
noremap   <Nop>
xnoremap # y?\V"
omap <silent> % <Plug>(MatchitOperationForward)
xmap <silent> % <Plug>(MatchitVisualForward)
nmap <silent> % <Plug>(MatchitNormalForward)
nnoremap & :&&
xnoremap * y/\V"
vnoremap < <gv
nnoremap < <<
vnoremap > >gv
nnoremap > >>
nnoremap H "mxh"mP
vnoremap J :m '>+1gv=gv
nnoremap J V:m '>+1gv=gv
vnoremap K :m '<-2gv=gv
nnoremap K V:m '<-2gv=gv
nnoremap L "mx"mp
nnoremap U 
nnoremap Y y$
omap <silent> [% <Plug>(MatchitOperationMultiBackward)
xmap <silent> [% <Plug>(MatchitVisualMultiBackward)
nmap <silent> [% <Plug>(MatchitNormalMultiBackward)
omap <silent> ]% <Plug>(MatchitOperationMultiForward)
xmap <silent> ]% <Plug>(MatchitVisualMultiForward)
nmap <silent> ]% <Plug>(MatchitNormalMultiForward)
xmap a% <Plug>(MatchitVisualTextObject)
xmap gx <Plug>NetrwBrowseXVis
nmap gx <Plug>NetrwBrowseX
omap <silent> g% <Plug>(MatchitOperationBackward)
xmap <silent> g% <Plug>(MatchitVisualBackward)
nmap <silent> g% <Plug>(MatchitNormalBackward)
nnoremap x "_x
nnoremap <Plug>PlenaryTestFile :lua require('plenary.test_harness').test_directory(vim.fn.expand("%:p"))
snoremap <silent> <Plug>luasnip-jump-prev <Cmd>lua require'luasnip'.jump(-1)
snoremap <silent> <Plug>luasnip-jump-next <Cmd>lua require'luasnip'.jump(1)
snoremap <silent> <Plug>luasnip-prev-choice <Cmd>lua require'luasnip'.change_choice(-1)
snoremap <silent> <Plug>luasnip-next-choice <Cmd>lua require'luasnip'.change_choice(1)
snoremap <silent> <Plug>luasnip-expand-snippet <Cmd>lua require'luasnip'.expand()
snoremap <silent> <Plug>luasnip-expand-or-jump <Cmd>lua require'luasnip'.expand_or_jump()
noremap <silent> <Plug>luasnip-expand-repeat <Cmd>lua require'luasnip'.expand_repeat()
noremap <silent> <Plug>luasnip-delete-check <Cmd>lua require'luasnip'.unlink_current_if_deleted()
xnoremap <Plug>(comment_toggle_blockwise_visual) <Cmd>lua require("Comment.api").locked("toggle.blockwise")(vim.fn.visualmode())
xnoremap <Plug>(comment_toggle_linewise_visual) <Cmd>lua require("Comment.api").locked("toggle.linewise")(vim.fn.visualmode())
tnoremap <silent> <Plug>(fzf-normal) 
tnoremap <silent> <Plug>(fzf-insert) i
nnoremap <silent> <Plug>(fzf-normal) <Nop>
nnoremap <silent> <Plug>(fzf-insert) i
xnoremap <silent> <Plug>NetrwBrowseXVis :call netrw#BrowseXVis()
nnoremap <silent> <Plug>NetrwBrowseX :call netrw#BrowseX(netrw#GX(),netrw#CheckIfRemote(netrw#GX()))
xmap <silent> <Plug>(MatchitVisualTextObject) <Plug>(MatchitVisualMultiBackward)o<Plug>(MatchitVisualMultiForward)
onoremap <silent> <Plug>(MatchitOperationMultiForward) :call matchit#MultiMatch("W",  "o")
onoremap <silent> <Plug>(MatchitOperationMultiBackward) :call matchit#MultiMatch("bW", "o")
xnoremap <silent> <Plug>(MatchitVisualMultiForward) :call matchit#MultiMatch("W",  "n")m'gv``
xnoremap <silent> <Plug>(MatchitVisualMultiBackward) :call matchit#MultiMatch("bW", "n")m'gv``
nnoremap <silent> <Plug>(MatchitNormalMultiForward) :call matchit#MultiMatch("W",  "n")
nnoremap <silent> <Plug>(MatchitNormalMultiBackward) :call matchit#MultiMatch("bW", "n")
onoremap <silent> <Plug>(MatchitOperationBackward) :call matchit#Match_wrapper('',0,'o')
onoremap <silent> <Plug>(MatchitOperationForward) :call matchit#Match_wrapper('',1,'o')
xnoremap <silent> <Plug>(MatchitVisualBackward) :call matchit#Match_wrapper('',0,'v')m'gv``
xnoremap <silent> <Plug>(MatchitVisualForward) :call matchit#Match_wrapper('',1,'v'):if col("''") != col("$") | exe ":normal! m'" | endifgv``
nnoremap <silent> <Plug>(MatchitNormalBackward) :call matchit#Match_wrapper('',0,'n')
nnoremap <silent> <Plug>(MatchitNormalForward) :call matchit#Match_wrapper('',1,'n')
tnoremap <C-W> <Cmd>tabclose
nnoremap <C-T> <Cmd>tabnew<Cmd>termA
tnoremap <C-Left> <Cmd>vertical resize -2
nnoremap <C-Left> <Cmd>vertical resize -2
tnoremap <C-Right> <Cmd>vertical resize +2
nnoremap <C-Right> <Cmd>vertical resize +2
tnoremap <C-Down> <Cmd>resize -2
nnoremap <C-Down> <Cmd>resize -2
tnoremap <C-Up> <Cmd>resize +2
nnoremap <C-Up> <Cmd>resize +2
tnoremap <C-9> <Cmd>tablast
nnoremap <C-9> <Cmd>tablast
tnoremap <C-8> <Cmd>norm 8gt
nnoremap <C-8> <Cmd>norm 8gt
tnoremap <C-7> <Cmd>norm 7gt
nnoremap <C-7> <Cmd>norm 7gt
tnoremap <C-6> <Cmd>norm 6gt
nnoremap <C-6> <Cmd>norm 6gt
tnoremap <C-5> <Cmd>norm 5gt
nnoremap <C-5> <Cmd>norm 5gt
tnoremap <C-4> <Cmd>norm 4gt
nnoremap <C-4> <Cmd>norm 4gt
tnoremap <C-3> <Cmd>norm 3gt
nnoremap <C-3> <Cmd>norm 3gt
tnoremap <C-2> <Cmd>norm 2gt
nnoremap <C-2> <Cmd>norm 2gt
tnoremap <C-1> <Cmd>norm 1gt
nnoremap <C-1> <Cmd>norm 1gt
tnoremap <C-L> <Cmd>wincmd l
tnoremap <C-K> <Cmd>wincmd k
nnoremap <C-K> <Cmd>wincmd k
tnoremap <C-J> <Cmd>wincmd j
nnoremap <C-J> <Cmd>wincmd j
tnoremap <C-H> <Cmd>wincmd h
nnoremap <C-H> <Cmd>wincmd h
nnoremap <C-W> <Cmd>close
nnoremap <C-S> <Cmd>w
nnoremap <C-L> <Cmd>wincmd l
inoremap  u
inoremap  u
let &cpo=s:cpo_save
unlet s:cpo_save
set completeopt=menu,menuone,preview,noinsert,noselect
set confirm
set copyindent
set formatoptions=crqn2lj
set guicursor=a:block,i:ver20,v:hor20,r-cr-o:hor20
set guifont=Fira\ Code\ NF:h16
set helplang=en
set ignorecase
set indentkeys=0{,0},!^F,o,O,0[,0]
set laststatus=3
set listchars=tab:│\ 
set mouse=
set operatorfunc=v:lua.require'Comment.api'.locked'toggle.linewise.current'
set preserveindent
set runtimepath=~/.config/nvim,/etc/xdg/nvim,~/.local/share/nvim/site,~/.local/share/nvim/site/pack/packer/start/packer.nvim,~/.local/share/nvim/site/pack/*/start/*,/usr/local/share/nvim/site,/usr/share/nvim/site,/usr/share/nvim/runtime,/usr/share/nvim/runtime/pack/dist/opt/matchit,/usr/lib/nvim,~/.local/share/nvim/site/pack/*/start/*/after,/usr/share/nvim/site/after,/usr/local/share/nvim/site/after,~/.local/share/nvim/site/after,/etc/xdg/nvim/after,~/.config/nvim/after,/usr/share/vim/vimfiles
set scrolloff=8
set shiftwidth=4
set noshowmode
set smartcase
set smartindent
set spelllang=en_us,de_de
set spelloptions=camel,noplainbuffer
set splitbelow
set splitright
set statusline=%{%v:lua.require'feline'.generate_statusline()%}
set noswapfile
set tabstop=4
set termguicolors
set textwidth=100
set undodir=~/.local/share/nvim/undo
set undofile
set updatetime=69
set winbar=%{%v:lua.require'feline'.generate_winbar()%}
set window=26
" vim: set ft=vim :
