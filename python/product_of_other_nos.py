#!/bin/python

def ListProduct(List):
        if len(List) >= 2:
                finalList = []
                totalItems = len(List)
                for number in range(0, totalItems, 1):
                        tempList = list(List)
                        tempList.pop(number)
                        product = 1
                        for item in tempList:
                                product = product*item
                        finalList.append(product)
                return finalList

List = [2,3,7]
print ListProduct(List)
