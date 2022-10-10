module Console
  class Widget
    def space_to_end(window)
      count = window.maxx - window.curx
      window << " " * count
    end
  end
end
