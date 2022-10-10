module Core
  class Cue
    attr_reader :name, :metadata

    def initialize(path)
      @path = path
      @name = path.basename.to_s
      @metadata = preprocess
    end

    def path
      @path.to_s
    end

    private

    def preprocess
      lines = File.readlines(@path)

      data = lines.map do |line|
        if line.start_with?("//")
          line = line[2..]
          parts = line.split(":")
          parts.map(&:strip)
        end
      end

      data.compact.to_h
    end
  end
end
