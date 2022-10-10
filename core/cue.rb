module Core
  class Cue
    attr_reader :name

    def initialize(path)
      @path = path
      @name = path.basename.to_s
    end

    def path
      @path.to_s
    end
  end
end
