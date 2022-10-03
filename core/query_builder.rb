class QueryBuilder
  def build(selector)
    query = []
    if selector.is_a?(Numeric)
      query << { type: :single, id: selector }
    elsif selector.is_a?(Range)
      query << { type: :range, start: selector.first, end: selector.last }
    end
  end
end
