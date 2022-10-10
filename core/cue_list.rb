require_relative "cue"

module Core
  class CueList
    attr_reader :cues

    def initialize(path)
      @path = path
      @cues = generate_cues
      @current = 0
    end

    def go
      @current += 1
      @current = 0 if @current >= @cues.length
    end

    # TODO: Cues should be able to named as well as numbered, so
    # we have to be more careful how we check for cues existance
    def goto(n)
      return [] if n >= @cues.length || @current == n


      old = @current
      @current = n

      return [true, [cue]] if @current == 0

      if n > old
        [false, files_to_run = @cues[old..n]]
      elsif n < old
        [true, @cues[..@current]]
      end
    end

    def cue
      @cues[@current]
    end

    def cue_is_before_current?(target_cue_path)
      index = @cues.find_index { |cue| cue.path == target_cue_path }
      return false if index.nil?

      index <= @current
    end

    def cue_exists?(target_cue_path)
      cue = @cues.find { |cue| cue.path == target_cue_path }

      !cue.nil?
    end

    def all_cues_till_current
      @cues[..@current]
    end

    def reload
      current_cue_path = @cues[@current].path

      @cues = generate_cues
      @current = @cues.find_index { |cue| cue.path == current_cue_path }
      # If you remove the current cue, just go to the first cue
      @current = 0 if @current.nil?
    end

    def to_s
      "CueList '#{@path.basename}' - Total: #{@cues.length} - Active: #{cue.name}"
    end

    private

    def generate_cues
      @path.children.sort.map { |child| Cue.new(child.expand_path) }
    end
  end
end
