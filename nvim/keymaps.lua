local keymaps = {}

function keymaps.setup()
  vim.keymap.set('n', '<leader>sta', function()
    vim.ui.input({
      prompt = "TopicLabels: ",
      default = "",
    }, function(topic)
      if not topic or topic == "" then return end
      
      vim.ui.input({
        prompt = "Task: ",
        default = "",
      }, function(task)
        if not task or task == "" then return end
        vim.fn.system("st add -T " .. vim.fn.shellescape(topic))
        vim.fn.system("st add -T " .. vim.fn.shellescape(topic) .. " -a " .. vim.fn.shellescape(task))
        if not vim.v.shell_error == 0 then
          vim.notify("st failed", vim.log.levels.ERROR)
        end
        local base = "st/topics/" .. topic .. "/" .. task .. "/"
        local files = {
          "DESC.md",
          "SHORT_DESC.md",
          "LABELS",
          "NOTES.md"
        }

        -- Open the first file in the current window
        vim.cmd("edit " .. base .. files[1])
        -- Open the rest in vertical splits
        for i = 2, #files do
          vim.cmd("vsplit " .. base .. files[i])
        end
      end)
    end)
  end, { desc = "Download layout from specific org." })

  vim.keymap.set('n', '<leader>stl', function()
    vim.ui.input({
      prompt = "Topic: ",
      default = "",
    }, function(topic)
      if not topic or topic == "" then return end
      
      vim.ui.input({
        prompt = "Labels: ",
        default = "",
      }, function(labels)
        local output
        if not labels or labels == "" then 
          output = vim.fn.system("st list -T " .. vim.fn.shellescape(topic) .. " -Mvn")
        else
          output = vim.fn.system("st list -T " .. vim.fn.shellescape(topic) .. " -Mvnl'" .. vim.fn.shellescape(labels) .. "'")
        end
        if vim.v.shell_error == 0 then
          vim.cmd("enew")
          local lines = vim.split(output, "\n")
          vim.api.nvim_buf_set_lines(0, 0, -1, false, lines)
          vim.bo.filetype = "markdown"
        else
          vim.notify("st failed", vim.log.levels.ERROR)
        end
      end)
    end)
  end, { desc = "Download layout from specific org." })
end
