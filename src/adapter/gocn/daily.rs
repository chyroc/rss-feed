pub async fn get_feed() {}

// func New(map[string]string) (*fetch.Source, error) {
// 	link := "https://gocn.vip/topics/cate/18"
// 	return &fetch.Source{
// 		Title:       "GoCN - 每日新闻",
// 		Description: "GoCN - 每日新闻",
// 		Link:        link,
//
// 		Fetch: func() (interface{}, error) {
// 			// text, err := helper.Req.New(http.MethodGet, link).Text()
// 			return nil, nil
// 		},
// 		Parse: func(obj interface{}) ([]*fetch.Item, error) {
// 			resp := new(getListResp)
// 			err := helper.Req.New(http.MethodGet, "https://gocn.vip/apiv3/topic/list?currentPage=1&cate2Id=18&grade=new").Unmarshal(resp)
// 			if err != nil {
// 				return nil, err
// 			}
//
// 			items := []*fetch.Item{}
// 			err = lambda.New(resp.Data.List).MapArrayAsync(func(idx int, v interface{}) interface{} {
// 				item := v.(*item)
// 				itemDetail := new(getItemDetailResp)
// 				_ = helper.Req.New(http.MethodGet, fmt.Sprintf("https://gocn.vip/apiv3/topic/%s/info", item.GUID)).Unmarshal(itemDetail)
// 				return &fetch.Item{
// 					Title:       item.Title,
// 					Link:        fmt.Sprintf("https://gocn.vip/topics/%s", item.GUID),
// 					Description: itemDetail.Data.Topic.ContentHTML,
// 					Author:      item.Nickname,
// 				}
// 			}).FilterList(func(idx int, obj interface{}) bool {
// 				return obj != nil && obj.(*fetch.Item) != nil
// 			}).ToObject(&items)
// 			if err != nil {
// 				return nil, err
// 			}
//
// 			return items, nil
// 		},
// 	}, nil
// }
//
// type getListResp struct {
// 	Code int    `json:"code"`
// 	Msg  string `json:"msg"`
// 	Data struct {
// 		List []*item `json:"list"`
// 	} `json:"data"`
// }
//
// type item struct {
// 	GUID       string `json:"guid"`
// 	UID        int    `json:"uid"`
// 	Nickname   string `json:"nickname"`
// 	Avatar     string `json:"avatar"`
// 	Title      string `json:"title"`
// 	Summary    string `json:"summary"`
// 	Ctime      int    `json:"ctime"`
// 	CntView    int    `json:"cntView"`
// 	Cate2ID    int    `json:"cate2Id"`
// 	Cate2Title string `json:"cate2Title"`
// 	CntLike    int    `json:"cntLike,omitempty"`
// 	CntCollect int    `json:"cntCollect,omitempty"`
// }
//
// type getItemDetailResp struct {
// 	Code int    `json:"code"`
// 	Msg  string `json:"msg"`
// 	Data struct {
// 		Tdk struct {
// 			Title       string `json:"title"`
// 			Keywords    string `json:"keywords"`
// 			Description string `json:"description"`
// 		} `json:"tdk"`
// 		Topic struct {
// 			GUID        string `json:"guid"`
// 			UID         int    `json:"uid"`
// 			Nickname    string `json:"nickname"`
// 			Avatar      string `json:"avatar"`
// 			Title       string `json:"title"`
// 			ContentHTML string `json:"contentHtml"`
// 			Ctime       int    `json:"ctime"`
// 			CntView     int    `json:"cntView"`
// 			Cate2ID     int    `json:"cate2Id"`
// 			Cate2Title  string `json:"cate2Title"`
// 		} `json:"topic"`
// 	} `json:"data"`
// }