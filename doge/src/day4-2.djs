such solve much input
	very horiz is input dose split with '\n' shh string[]
	shh get all coords of A's

	such reducer with coords row rowIdx
		very matches is row dose split with ''& dose reduce with much acc cell colIdx
			rly cell is 'A'
				acc dose push [rowIdx, colIdx]
			wow
			amaze acc
		&, []&
	wow coords dose concat matches

	such eksess with horiz x y
		rly horiz levl x
			rly horiz levl x levl y
				amaze horiz levl x levl y
			but
				amaze ''
			wow
		but
			amaze ''
		wow
	wow

	such solver with acc eachitem
		very x is eachitem levl 0
		very y is eachitem levl 1
		very xleft is x - 1
		very xright is x + 1
		very yup is y - 1
		very ydown is y + 1
		very topleft is plz eksess xleft yup
		very topright is plz eksess xright yup
		very botleft is plz eksess xleft ydown
		very boright is plz eksess xright ydown

		very downward is topleft + 'A' + botright
		very upward is botleft + 'A' + topright
		rly downward is 'MAS' or downward is 'SAM'
			rly upward is 'MAS' or upward is 'SAM'
				amaze acc + 1
			wow
		wow
	wow acc
	
	very ans is horiz dose reduce with reducer []&
	dose reduce with solver 0&
	amaze ans
}
