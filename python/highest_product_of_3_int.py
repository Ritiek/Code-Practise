#!/bin/python

""" I have modified this case from any top 3 ints to any top 'n' ints..
  Use times = 3 for top 3 ints.  """

def highestProduct(List, times):
        top_int = []
        product = 1
        for time in range(0, times, 1):
                top_int.append(max(List))
                List.remove(max(List))
        for number in top_int:
                product = product * number
        return product

List = [3,6,3,4,3,5]
times = int(raw_input('How many nos. to product through: '))
print List
print highestProduct(List, times)
