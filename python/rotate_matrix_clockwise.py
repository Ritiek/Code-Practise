#!/usr/bin/env python

""" Example:
given matrix = [[1,2,3],
                [4,5,6],
                [7,8,9]]
returned mat = [[7,4,1],
                [8,5,2],
                [9,6,3]] """


def rotateClockwise(mat):
	for y in range(0, len(mat)):
		horizontal = []
		for x in range(0, len(mat[0])):
			horizontal.append(mat[len(mat)-1-x][y])
		new_mat.append(horizontal)
	return new_mat
		
mat = [[1,2,3],
       [4,5,6],
       [7,8,9]]
	   
print rotateClockwise(mat)
