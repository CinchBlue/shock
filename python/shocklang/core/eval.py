from shocklang.core.graph.data import Place, GraphStore


class BasicEvaluator:
    """Evaluates basic arithmetic expressions in place graph form"""

    def __init__(self):
        pass

    def evaluate(self, root: Place, graph: GraphStore) -> int:
        # print("Place: {}".format(root))
        final_value = 0
        if root.attr["primtype"] == "Procedure":
            operands = graph.get_links_from(root.id)
            for link in operands.values():
                # print("Link: {}".format(link))
                if link.name.startswith("op"):
                    op_result = self.evaluate(
                        graph.get_place(link.dest_place_id),
                        graph)
                    if root.attr["name"] == "+":
                        final_value += op_result
            return final_value
        elif root.attr["primtype"] == "Integer":
            return root.attr["value"]
        else:
            raise RuntimeError("What the duck this wasn't a Proc or Int!")
