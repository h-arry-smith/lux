module Core
  class QueryBuilder
    def build(selector)
      query = []
      if selector.is_a?(StaticValue)
        query << { type: :single, id: selector.value }
      elsif selector.is_a?(ValueRange)
        query << { type: :range, start: selector.start, end: selector.finish }
      end
    end
  end
end
