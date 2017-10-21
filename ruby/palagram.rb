#!/usr/bin/env ruby

def anagram(s)
  odd = false
  s_squeeze = s.chars.sort.join.squeeze
  s_squeeze.chars.sort.each { |c|
    appnc = s.count(c)
    if appnc % 2 == 1
        if odd
          return "NO"
        end
        odd = true
    end
  }
  return "YES"
end

s = gets.strip
result = anagram(s)
puts result
