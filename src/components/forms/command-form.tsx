import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export default function CommandForm() {
  return (
    <div className="p-4 border rounded-lg">
      <h2 className="text-xl font-semibold mb-4">Command Control</h2>
      <form className="space-y-4">
        <div>
          <label htmlFor="test1" className="block text-sm font-medium text-gray-700">
            Test Item 1
          </label>
          <Input type="text" id="test1" placeholder="Enter value for test 1" />
        </div>
        <div>
          <label htmlFor="test2" className="block text-sm font-medium text-gray-700">
            Test Item 2
          </label>
          <Input type="text" id="test2" placeholder="Enter value for test 2" />
        </div>
        <div className="flex justify-end space-x-2">
          <Button type="submit">Submit</Button>
          <Button type="button" variant="outline">Cancel</Button>
        </div>
      </form>
    </div>
  );
}