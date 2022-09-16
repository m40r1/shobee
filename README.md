#shobee

* PROJ refazer main
** [?] Talvez nao usar varios vec<dado> e sim um vec<response>
mapear uma respota da api
usar ela pra mapear uma colecao unica no mongodb
talvez seja pior para o serde(serializar e deserializar dados)


* IDEA checar categoriaid para n fugir mt dos items
** [ ] mapear a categoria para uma struct
** HOLD  caso eu decida procurar e passas os items dentro da loja


* TODO varios headers
** [ ] criar struct para header map
** [ ] usar impls para setar diferentes combinacoes
#+begin_example
impl header{
firefox()
android()
etc...
}
#+end_example
** [?] melhor tipo de struct pra valor unico
** [X] lidando com tipos
** [ ] parse dos  headers do arquivo json
[[file:~/data/org/user_agent.json]]



* KILL Lidar com erros
** [-] olhar match de padroes
** [-] erros mais "graciosos"
** [-] lidar com mais cenarios
making it compile is just the basic
it needs to be resilient

* TODO Voucher
** [?] Olhar informacoes util
** [ ] atualizar  modelo voucher
** TODO motivo de usar o voucher????


* PROJ Lidar com flash_sale
** [ ] Capped collection
um tipo de colecao no mongodb para dados que n precisam ser guardados por mt tempo
** TODO 24hrs bot???
olha o tempo que cada compra da flash_sale acaba
pra saber de quanto em quanto tempo ele precisa de uma versao nova da pagina
** IDEA sera necessario que ele saiba as horas/tenha um timer?

* Apis
** https://shopee.com.br/api/v4/product/get_shop_info?shopid=344038694

** https://shopee.com.br/api/v4/shop/get_shop_detail?sort_sold_out=0&username=computer_ram.br

** https://shopee.com.br/api/v4/shop/get_categories?limit=20&offset=0&shopid=344038694&version=2

** https://shopee.com.br/api/v4/shop/search_items?filter_sold_out=2&limit=6&offset=0&order=desc&shopid=344038694&sort_by=sales&use_case=1

** https://shopee.com.br/api/v2/voucher_wallet/get_shop_vouchers_by_shopid?shopid=344038694&with_claiming_status=true

** https://shopee.com.br/api/v4/recommend/recommend?bundle=shop_page_category_tab_main&item_card=2&limit=30&offset=0&section=shop_page_category_tab_main_sec&shopid=344038694&sort_type=1&tab_name=popular&upstream=pdp

** https://shopee.com.br/api/v4/search/search_items?by=relevancy&keyword=ssd&limit=60&newest=0&order=desc&page_type=search&scenario=PAGE_GLOBAL_SEARCH&version=2
